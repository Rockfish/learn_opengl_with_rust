#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(unused_assignments)]
#![allow(unused_variables)]

use crate::mesh::{Mesh, Texture, Vertex};
use crate::shader_m::Shader_M;
use glam::*;
use russimp::mesh;
use russimp::node::*;
use russimp::scene::*;
use russimp::material::*;
use std::path::Path;
use std::rc::Rc;

// model data
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
                // PostProcess::SortByPrimitiveType
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
                self.process_node(&scene.root, &scene);
            }
            Err(err) => panic!("{}", err),
        }
    }

    fn process_node(&mut self, root: &Option<Rc<Node>>, scene: &Scene) {
        // process each mesh located at the current node
        if let Some(node) = root {
            for mesh_id in &node.meshes {
                let scene_mesh = &scene.meshes[*mesh_id as usize];
                let mesh = Model::process_mesh(scene_mesh, scene);
            }
        }
    }

    fn process_mesh(scene_mesh: &russimp::mesh::Mesh, scene: &Scene) -> Mesh {
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
                    scene_mesh.normals[i].x,
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
        let diffuseMaps = Model::loadMaterialTextures(material, TextureType::Diffuse, "texture_diffuse");
        textures.extend(diffuseMaps);
        // 2. specular maps
        let specularMaps = Model::loadMaterialTextures(material, TextureType::Specular, "texture_specular");
        textures.extend(specularMaps);
        // 3. normal maps
        let normalMaps = Model::loadMaterialTextures(material, TextureType::Height, "texture_normal");
        textures.extend(normalMaps);
        // 4. height maps
        let heightMaps = Model::loadMaterialTextures(material, TextureType::Ambient, "texture_height");
        textures.extend(heightMaps);

         Mesh {
            vertices: vertices,
            indices: indices,
            textures: textures,
            VAO: 0,
        }
    }

    fn loadMaterialTextures(mat: &Material, texture_type: TextureType, typeName: &str) -> Vec<Texture> {
        let mut textures: Vec<Texture> = vec![];

        for i in 0..mat.textures.get(texture_type)


        textures
    }
}
