#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(unused_assignments)]
#![allow(unused_variables)]

use crate::shader_m::Shader_M;
use glad_gl::gl;
use glad_gl::gl::{GLsizei, GLsizeiptr, GLuint, GLvoid};
use glam::*;
use std::ffi::CString;
use std::mem;

use crate::SIZE_OF_FLOAT;

const MAX_BONE_INFLUENCE: usize = 4;

#[derive(Debug, Copy, Clone)]
#[repr(C, packed)]
pub struct Vertex {
    pub Position: Vec3,
    pub Normal: Vec3,
    pub TexCoords: Vec2,
    pub Tangent: Vec3,
    pub Bitangent: Vec3,
    pub m_BoneIDs: [i32; MAX_BONE_INFLUENCE],
    pub m_Weights: [f32; MAX_BONE_INFLUENCE],
}

impl Vertex {
    pub fn new() -> Vertex {
        Vertex {
            Position: Default::default(),
            Normal: Default::default(),
            TexCoords: Default::default(),
            Tangent: Default::default(),
            Bitangent: Default::default(),
            m_BoneIDs: [0; MAX_BONE_INFLUENCE],
            m_Weights: [0.0; MAX_BONE_INFLUENCE],
        }
    }
}

// const OFFSET_OF_NORMAL: usize = mem::size_of::<Vec3>();
// const OFFSET_OF_TEXCOORDS: usize = OFFSET_OF_NORMAL + mem::size_of::<Vec3>();
// const OFFSET_OF_TANGENT: usize = OFFSET_OF_TEXCOORDS + mem::size_of::<Vec2>();
// const OFFSET_OF_BITANGENT: usize = OFFSET_OF_TANGENT + mem::size_of::<Vec3>();
// const OFFSET_OF_BONE_IDS: usize = OFFSET_OF_BITANGENT + mem::size_of::<Vec3>();
// const OFFSET_OF_WEIGHTS: usize = OFFSET_OF_BONE_IDS + mem::size_of::<i32>()  * MAX_BONE_INFLUENCE;

const OFFSET_OF_NORMAL: usize = mem::offset_of!(Vertex, Normal);
const OFFSET_OF_TEXCOORDS: usize = mem::offset_of!(Vertex, TexCoords);
const OFFSET_OF_TANGENT: usize = mem::offset_of!(Vertex, Tangent);
const OFFSET_OF_BITANGENT: usize = mem::offset_of!(Vertex, Bitangent);
const OFFSET_OF_BONE_IDS: usize = mem::offset_of!(Vertex, m_BoneIDs);
const OFFSET_OF_WEIGHTS: usize = mem::offset_of!(Vertex, m_Weights);

#[derive(Debug, Clone)]
pub struct Texture {
    pub id: u32,
    pub texture_type: String,
    pub path: String,
}

impl Texture {
    pub fn new() -> Texture {
        Texture {
            id: 0,
            texture_type: "".to_string(),
            path: "".to_string(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Mesh {
    pub vertices: Vec<Vertex>,
    pub indices: Vec<u32>,
    pub textures: Vec<Texture>,
    pub VAO: u32,
}

impl Mesh {
    pub fn new(vertices: Vec<Vertex>, indices: Vec<u32>, textures: Vec<Texture>) -> Mesh {
        let mut mesh = Mesh {
            vertices,
            indices,
            textures,
            VAO: 99999,
        };
        mesh.setupMesh();
        mesh
    }

    pub fn Draw(&self, shader: &Shader_M) {
        // bind appropriate textures
        let mut diffuseNr: u32 = 0;
        let mut specularNr: u32 = 0;
        let mut normalNr: u32 = 0;
        let mut heightNr: u32 = 0;

        unsafe {
            for (i, texture) in self.textures.iter().enumerate() {
                gl::ActiveTexture(gl::TEXTURE0 + i as u32); // active proper texture unit before binding

                // retrieve texture number (the N in diffuse_textureN)
                let number = if texture.texture_type == "texture_diffuse" {
                    diffuseNr += 1;
                    diffuseNr.to_string()
                } else if texture.texture_type == "texture_specular" {
                    specularNr += 1;
                    specularNr.to_string()
                } else if texture.texture_type == "texture_normal" {
                    normalNr += 1;
                    normalNr.to_string()
                } else if texture.texture_type == "texture_height" {
                    heightNr += 1;
                    heightNr.to_string()
                } else {
                    panic!("Unknown texture type")
                };

                // now set the sampler to the correct texture unit
                let mut name = texture.texture_type.clone();
                name.push_str(&number);
                let c_string = CString::new(name).unwrap();
                gl::Uniform1i(
                    gl::GetUniformLocation(shader.programId, c_string.as_ptr()),
                    i as i32,
                );
                // and finally bind the texture
                gl::BindTexture(gl::TEXTURE_2D, texture.id);
            }

            gl::BindVertexArray(self.VAO);
            gl::DrawElements(gl::TRIANGLES, self.indices.len() as i32, gl::UNSIGNED_INT, 0 as *const GLvoid);
            gl::BindVertexArray(0);
        }
    }

    fn setupMesh(&mut self) {
        let mut VBO: GLuint = 0;
        let mut EBO: GLuint = 0;

        unsafe {
            gl::GenVertexArrays(1, &mut self.VAO);
            gl::GenBuffers(1, &mut VBO);
            gl::GenBuffers(1, &mut EBO);

            gl::BindVertexArray(self.VAO);
            // load data into vertex buffers
            gl::BindBuffer(gl::ARRAY_BUFFER, VBO);
            // A great thing about structs is that their memory layout is sequential for all its items. (original comment from cpp code)
            // The effect is that we can simply pass a pointer to the struct and it translates perfectly to a glm::vec3/2 array which
            // again translates to 3/2 floats which translates to a byte array.
            gl::BufferData(
                gl::ARRAY_BUFFER,
                (self.vertices.len() * mem::size_of::<Vertex>()) as GLsizeiptr,
                self.vertices.as_ptr() as *const GLvoid,
                gl::STATIC_DRAW,
            );

            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, EBO);
            gl::BufferData(
                gl::ELEMENT_ARRAY_BUFFER,
                (self.indices.len() *  mem::size_of::<u32>()) as GLsizeiptr,
                self.indices.as_ptr() as *const GLvoid,
                gl::STATIC_DRAW,
            );

            // set the vertex attribute pointers
            // vertex Positions
            gl::EnableVertexAttribArray(0);
            gl::VertexAttribPointer(
                0,
                3,
                gl::FLOAT,
                gl::FALSE,
                mem::size_of::<Vertex>() as GLsizei,
                0 as *const GLvoid,
            );

            // vertex normals
            gl::EnableVertexAttribArray(1);
            gl::VertexAttribPointer(
                1,
                3,
                gl::FLOAT,
                gl::FALSE,
                mem::size_of::<Vertex>() as GLsizei,
                (OFFSET_OF_NORMAL) as *const GLvoid,
            );

            // vertex texture coordinates
            gl::EnableVertexAttribArray(2);
            gl::VertexAttribPointer(
                2,
                3,
                gl::FLOAT,
                gl::FALSE,
                mem::size_of::<Vertex>() as GLsizei,
                (OFFSET_OF_TEXCOORDS) as *const GLvoid,
            );

            // vertex tangent
            gl::EnableVertexAttribArray(3);
            gl::VertexAttribPointer(
                3,
                3,
                gl::FLOAT,
                gl::FALSE,
                mem::size_of::<Vertex>() as GLsizei,
                (OFFSET_OF_TANGENT) as *const GLvoid,
            );

            // vertex bitangent
            gl::EnableVertexAttribArray(4);
            gl::VertexAttribPointer(
                4,
                3,
                gl::FLOAT,
                gl::FALSE,
                mem::size_of::<Vertex>() as GLsizei,
                (OFFSET_OF_BITANGENT) as *const GLvoid,
            );

            // ids
            gl::EnableVertexAttribArray(5);
            gl::VertexAttribPointer(
                5,
                4,
                gl::FLOAT,
                gl::FALSE,
                mem::size_of::<Vertex>() as GLsizei,
                (OFFSET_OF_BONE_IDS) as *const GLvoid,
            );

            // weights
            gl::EnableVertexAttribArray(6);
            gl::VertexAttribPointer(
                6,
                4,
                gl::FLOAT,
                gl::FALSE,
                mem::size_of::<Vertex>() as GLsizei,
                (OFFSET_OF_WEIGHTS) as *const GLvoid,
            );

            gl::BindVertexArray(0);
        }
    }
}