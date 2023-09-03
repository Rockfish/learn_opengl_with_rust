#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(unused_assignments)]

use glad_gl::gl;
use glad_gl::gl::{GLchar, GLint};
use glam::*;

use std::fs::File;
use std::io::prelude::*;
use std::io::Error;
use std::path::Path;
use std::ptr;

use crate::*;

type ID = u32;

pub struct Shader_M {
    pub programId: ID,
}

impl Shader_M {
    pub fn new() -> Shader_M {
        Shader_M { programId: 0 }
    }

    pub fn build(&mut self, vertexPath: &str, fragmentPath: &str) -> Result<ID, Error> {
        let mut vertexCode: String = Default::default();
        let mut fragmentCode: String = Default::default();

        // open files
        let vertexPath = Path::new(vertexPath);
        let fragmentPath = Path::new(fragmentPath);

        let mut vShaderFile = File::open(vertexPath)?;
        let mut fShaderFile = File::open(fragmentPath)?;

        vShaderFile.read_to_string(&mut vertexCode)?;
        fShaderFile.read_to_string(&mut fragmentCode)?;

        unsafe {
            // vertex shader
            let vertexShader = gl::CreateShader(gl::VERTEX_SHADER);
            let fragmentShader = gl::CreateShader(gl::FRAGMENT_SHADER);
            self.programId = gl::CreateProgram();

            let c_string = c_string!(vertexCode);
            gl::ShaderSource(vertexShader, 1, &c_string.as_ptr(), ptr::null());
            gl::CompileShader(vertexShader);
            checkCompileErrors(vertexShader, "VERTEX");

            let c_string = c_string!(fragmentCode);
            gl::ShaderSource(fragmentShader, 1, &c_string.as_ptr(), ptr::null());
            gl::CompileShader(fragmentShader);
            checkCompileErrors(fragmentShader, "VERTEX");

            // link the first program object
            gl::AttachShader(self.programId, vertexShader);
            gl::AttachShader(self.programId, fragmentShader);
            gl::LinkProgram(self.programId);
            checkCompileErrors(self.programId, "PROGRAM");

            // Now that the shader programs have been built we can free up memory by deleting the shaders.
            gl::DeleteShader(vertexShader);
            gl::DeleteShader(fragmentShader);
        }

        Ok(self.programId)
    }

    pub fn use_shader(&self) {
        unsafe {
            gl::UseProgram(self.programId);
        }
    }

    // utility uniform functions
    // ------------------------------------------------------------------------
    pub fn setBool(&self, name: &str, value: bool) {
        unsafe {
            let v = if value { 1 } else { 0 };
            let c_string = c_string!(name);
            let location = gl::GetUniformLocation(self.programId, c_string.as_ptr());
            gl::Uniform1i(location, v);
        }
    }

    // ------------------------------------------------------------------------
    pub fn setInt(&self, name: &str, value: i32) {
        unsafe {
            let c_string = c_string!(name);
            let location = gl::GetUniformLocation(self.programId, c_string.as_ptr());
            gl::Uniform1i(location, value);
        }
    }

    // ------------------------------------------------------------------------
    pub fn setFloat(&self, name: &str, value: f32) {
        unsafe {
            let c_string = c_string!(name);
            let location = gl::GetUniformLocation(self.programId, c_string.as_ptr());
            gl::Uniform1f(location, value);
        }
    }

    // ------------------------------------------------------------------------
    pub fn setVec2(&self, name: &str, value: &Vec2) {
        unsafe {
            let c_string = c_string!(name);
            let location = gl::GetUniformLocation(self.programId, c_string.as_ptr());
            gl::Uniform2fv(location, 1, value.to_array().as_ptr());
        }
    }

    // ------------------------------------------------------------------------
    pub fn setVec2_xy(&self, name: &str, x: f32, y: f32) {
        unsafe {
            let c_string = c_string!(name);
            let location = gl::GetUniformLocation(self.programId, c_string.as_ptr());
            gl::Uniform2f(location, x, y);
        }
    }

    // ------------------------------------------------------------------------
    pub fn setVec3(&self, name: &str, value: &Vec3) {
        unsafe {
            let c_string = c_string!(name);
            let location = gl::GetUniformLocation(self.programId, c_string.as_ptr());
            gl::Uniform3fv(location, 1, value.to_array().as_ptr());
        }
    }

    // ------------------------------------------------------------------------
    pub fn setVec3_xyz(&self, name: &str, x: f32, y: f32, z: f32) {
        unsafe {
            let c_string = c_string!(name);
            let location = gl::GetUniformLocation(self.programId, c_string.as_ptr());
            gl::Uniform3f(location, x, y, z);
        }
    }

    // ------------------------------------------------------------------------
    pub fn setVec4(&self, name: &str, value: &Vec4) {
        unsafe {
            let c_string = c_string!(name);
            let location = gl::GetUniformLocation(self.programId, c_string.as_ptr());
            gl::Uniform4fv(location, 1, value.to_array().as_ptr());
        }
    }

    // ------------------------------------------------------------------------
    pub fn setVec4_xyzw(&self, name: &str, x: f32, y: f32, z: f32, w: f32) {
        unsafe {
            let c_string = c_string!(name);
            let location = gl::GetUniformLocation(self.programId, c_string.as_ptr());
            gl::Uniform4f(location, x, y, z, w);
        }
    }

    // ------------------------------------------------------------------------
    pub fn setMat2(&self, name: &str, mat: &Mat2) {
        unsafe {
            let c_string = c_string!(name);
            let location = gl::GetUniformLocation(self.programId, c_string.as_ptr());
            gl::UniformMatrix2fv(location, 1, gl::FALSE, mat.to_cols_array().as_ptr());
        }
    }

    // ------------------------------------------------------------------------
    pub fn setMat3(&self, name: &str, mat: &Mat3) {
        unsafe {
            let c_string = c_string!(name);
            let location = gl::GetUniformLocation(self.programId, c_string.as_ptr());
            gl::UniformMatrix3fv(location, 1, gl::FALSE, mat.to_cols_array().as_ptr());
        }
    }

    // ------------------------------------------------------------------------
    pub fn setMat4(&self, name: &str, matrix: &Mat4) {
        unsafe {
            let c_string = c_string!(name);
            let location = gl::GetUniformLocation(self.programId, c_string.as_ptr());
            gl::UniformMatrix4fv(location, 1, gl::FALSE, matrix.to_cols_array().as_ptr());
        }
    }
}

fn checkCompileErrors(shaderId: u32, checkType: &str) {
    unsafe {
        let mut status = gl::FALSE as GLint;

        if checkType != "PROGRAM" {
            gl::GetShaderiv(shaderId, gl::COMPILE_STATUS, &mut status);
            if status != (gl::TRUE as GLint) {
                let mut len = 0;
                gl::GetShaderiv(shaderId, gl::INFO_LOG_LENGTH, &mut len);
                // Subtract 1 to skip the trailing null character.
                let mut infoLog = vec![0; len as usize - 1];
                gl::GetProgramInfoLog(shaderId, 1024, ptr::null_mut(), infoLog.as_mut_ptr() as *mut GLchar);
                panic!("Shader compilation failed.\n{}", String::from_utf8_lossy(&infoLog));
            }
        } else {
            gl::GetProgramiv(shaderId, gl::LINK_STATUS, &mut status);
            if status != (gl::TRUE as GLint) {
                let mut len = 0;
                gl::GetProgramiv(shaderId, gl::INFO_LOG_LENGTH, &mut len);
                // Subtract 1 to skip the trailing null character.
                let mut infoLog = vec![0; len as usize - 1];
                gl::GetProgramInfoLog(shaderId, 1024, ptr::null_mut(), infoLog.as_mut_ptr() as *mut GLchar);
                panic!("Shader program linking failed.\n{}", String::from_utf8_lossy(&infoLog));
            }
        }
    }
}
