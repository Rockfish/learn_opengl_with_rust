#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(unused_assignments)]
#![allow(unused_variables)]

use std::ops::Deref;
use crate::mesh::{Mesh, Texture, Vertex};
use crate::shader_m::Shader_M;
use glam::*;
use russimp::node::*;
use russimp::scene::*;
use russimp::material::*;
use std::path::Path;
use std::rc::Rc;
use glad_gl::gl;
use glad_gl::gl::{GLint, GLsizei, GLuint, GLvoid};
use image::ColorType;

// model data
#[derive(Debug)]
pub struct Model {
    // stores all the textures loaded so far, optimization to make sure textures aren't loaded more than once.
    pub textures_loaded: Vec<Texture>,
    pub meshes: Vec<Mesh>,
    pub directory: String,
    pub gammaCorrection: bool,
}

impl Model {
    pub fn new(path: &str, gamma: bool) -> Model {
        let mut model = Model {
            textures_loaded: vec![],
            meshes: vec![],
            directory: "".to_string(),
            gammaCorrection: gamma,
        };
        model.load_model(path);
        model
    }

    pub fn Draw(&self, shader: &Shader_M) {
        for mesh in &self.meshes {
            mesh.Draw(shader);
        }
    }

    // loads a model with supported ASSIMP extensions from file and stores the resulting meshes in the meshes vector.
    fn load_model(&mut self, path: &str) {
        let scene = Scene::from_file(
            &path,
            vec![
                PostProcess::Triangulate,
                PostProcess::GenerateSmoothNormals,
                PostProcess::FlipUVs,
                PostProcess::CalculateTangentSpace,
                // PostProcess::JoinIdenticalVertices,
                // PostProcess::SortByPrimitiveType,
                // PostProcess::EmbedTextures,
            ],
        );

        match scene {
            Ok(scene) => {
                self.directory = Path::new(path)
                    .parent()
                    .expect("path error")
                    .to_str()
                    .unwrap()
                    .to_string();
                if let Some(node) = &scene.root {
                    self.process_node(node, &scene);
                }
            }
            Err(err) => panic!("{}", err),
        }
        // println!("Model:\n{:#?}", self);
    }

    fn process_node(&mut self, node: &Rc<Node>, scene: &Scene) {
        // process each mesh located at the current node
        println!("{:?}", node.name);
        // println!("{:#?}", node);
        for mesh_id in &node.meshes {
            let scene_mesh = &scene.meshes[*mesh_id as usize];
            let mesh = self.process_mesh(scene_mesh, scene);
            self.meshes.push(mesh);
        }
        let childern = node.children.take();
        for child in childern {
            println!("{:#?}", child.name);
            self.process_node(&child, scene);
        }
    }

    fn process_mesh(&mut self, scene_mesh: &russimp::mesh::Mesh, scene: &Scene) -> Mesh {
        let mut vertices: Vec<Vertex> = vec![];
        let mut indices: Vec<u32> = vec![];
        let mut textures: Vec<Texture> = vec![];

        for i in 0..scene_mesh.vertices.len() {
            let mut vertex = Vertex::new();

            // positions
            vertex.Position = vec3(
                scene_mesh.vertices[i].x,
                scene_mesh.vertices[i].y,
                scene_mesh.vertices[i].z,
            );
            // normals
            if !scene_mesh.normals.is_empty() {
                vertex.Normal = vec3(
                    scene_mesh.normals[i].x,
                    scene_mesh.normals[i].y,
                    scene_mesh.normals[i].z,
                );
            }
            // texture coordinates
            if !scene_mesh.texture_coords.is_empty() {
                // a vertex can contain up to 8 different texture coordinates. We thus make the assumption that we won't
                // use models where a vertex can have multiple texture coordinates so we always take the first set (0).
                if let Some(coord) = &scene_mesh.texture_coords[0] {
                    vertex.TexCoords = vec2(
                        coord[i].x,
                        coord[i].y,
                    );
                }
                // tangent
                vertex.Tangent = vec3(
                    scene_mesh.tangents[i].x,
                    scene_mesh.tangents[i].y,
                    scene_mesh.tangents[i].z,
                );
                // bitangent
                vertex.Bitangent = vec3(
                    scene_mesh.bitangents[i].x,
                    scene_mesh.bitangents[i].y,
                    scene_mesh.bitangents[i].z,
                );
            } else {
                vertex.TexCoords = vec2(0.0, 0.0);
            }

            vertices.push(vertex);
        }
        // now walk through each of the mesh's faces (a face is a mesh its triangle) and retrieve the corresponding vertex indices.
        for i in 0..scene_mesh.faces.len() {
            let face = &scene_mesh.faces[i];
            indices.extend(face.0.iter());
        }

        // process materials
        // aiMaterial* material = scene->mMaterials[mesh->mMaterialIndex];
        let material = &scene.materials[scene_mesh.material_index as usize];
        // we assume a convention for sampler names in the shaders. Each diffuse texture should be named
        // as 'texture_diffuseN' where N is a sequential number ranging from 1 to MAX_SAMPLER_NUMBER.
        // Same applies to other texture as the following list summarizes:
        // diffuse: texture_diffuseN
        // specular: texture_specularN
        // normal: texture_normalN

        // 1. diffuse maps
        let diffuseMaps = self.loadMaterialTextures(material, TextureType::Diffuse, "texture_diffuse");
        textures.extend(diffuseMaps);
        // 2. specular maps
        let specularMaps = self.loadMaterialTextures(material, TextureType::Specular, "texture_specular");
        textures.extend(specularMaps);
        // 3. normal maps
        let normalMaps = self.loadMaterialTextures(material, TextureType::Height, "texture_normal");
        textures.extend(normalMaps);
        // 4. height maps
        let heightMaps = self.loadMaterialTextures(material, TextureType::Ambient, "texture_height");
        textures.extend(heightMaps);

        let mesh = Mesh::new(vertices, indices, textures);
        mesh
    }

    fn loadMaterialTextures(&mut self, mat: &Material, texture_type: TextureType, typeName: &str) -> Vec<Texture> {
        let mut textures: Vec<Texture> = vec![];

        if let Some(text) = mat.textures.get(&texture_type) {
            let filename = text.deref().borrow().filename.clone();

            let loaded_texture = self.textures_loaded.iter().find(|t| t.path == filename);

            if let Some(texture) = loaded_texture {
                textures.push(texture.clone());
            } else {
                let filepath = format!("{}/{}", self.directory, filename);
                let id = self.textureFromFile(&filepath);
                let texture = Texture {
                    id: id,
                    texture_type: typeName.to_string(),
                    path: filename,
                };
                textures.push(texture.clone());
                self.textures_loaded.push(texture);
            }
        }

        /* hack
        let texture_name = match texture_type {
            TextureType::Diffuse => "diffuse.jpg",
            TextureType::Specular => "specular.jpg",
            TextureType::Height => "normal.png",
            _ => ""
        };

        if !texture_name.is_empty() {
            let filename = texture_name;

            let loaded_texture = self.textures_loaded.iter().find(|t| t.path == filename);

            if let Some(texture) = loaded_texture {
                textures.push(texture.clone());
            } else {
                let filepath = format!("{}/{}", self.directory, filename);
                let id = self.textureFromFile(&filepath);
                let texture = Texture {
                    id: id,
                    texture_type: typeName.to_string(),
                    path: filename.to_string(),
                };
                textures.push(texture.clone());
                self.textures_loaded.push(texture);
            }
        }
         */

        textures
    }

    fn textureFromFile(&self, filepath: &str) -> u32 {
        let mut texture_id: GLuint = 0;

        let img = image::open(filepath).expect("Texture failed to load");
        let (width, height) = (img.width() as GLsizei, img.height() as GLsizei);

        let color_type = img.color();

        unsafe {
            let format = match color_type {
                ColorType::L8 => gl::RED,
                // ColorType::La8 => {}
                ColorType::Rgb8 => gl::RGB,
                ColorType::Rgba8 => gl::RGBA,
                // ColorType::L16 => {}
                // ColorType::La16 => {}
                // ColorType::Rgb16 => {}
                // ColorType::Rgba16 => {}
                // ColorType::Rgb32F => {}
                // ColorType::Rgba32F => {}
                _ => panic!("no mapping for color type"),
            };

            let data = match color_type {
                ColorType::L8 => img.into_rgb8().into_raw(),
                // ColorType::La8 => {}
                ColorType::Rgb8 => img.into_rgb8().into_raw(),
                ColorType::Rgba8 => img.into_rgba8().into_raw(),
                // ColorType::L16 => {}
                // ColorType::La16 => {}
                // ColorType::Rgb16 => {}
                // ColorType::Rgba16 => {}
                // ColorType::Rgb32F => {}
                // ColorType::Rgba32F => {}
                _ => panic!("no mapping for color type"),
            };

            gl::GenTextures(1, &mut texture_id);
            gl::BindTexture(gl::TEXTURE_2D, texture_id);

            gl::TexImage2D(
                gl::TEXTURE_2D,
                0,
                format as GLint,
                width,
                height,
                0,
                format,
                gl::UNSIGNED_BYTE,
                data.as_ptr() as *const GLvoid,
            );
            gl::GenerateMipmap(gl::TEXTURE_2D);

            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT as GLint);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT as GLint);

            gl::TexParameteri(
                gl::TEXTURE_2D,
                gl::TEXTURE_MIN_FILTER,
                gl::LINEAR_MIPMAP_LINEAR as GLint,
            );
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as GLint);
        }

        texture_id
    }
}

