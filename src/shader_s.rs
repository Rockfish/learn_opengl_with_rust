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

pub struct Shader_S {
    pub programId: ID,
}

impl Shader_S {
    pub fn new(vertexPath: &str, fragmentPath: &str) -> Result<Self, String> {
        let mut shader = Shader_S { programId: 0 };
        let mut vertexCode: String = Default::default();
        let mut fragmentCode: String = Default::default();

        match read_file(vertexPath) {
            Ok(content) => vertexCode = content,
            Err(error) => return Err(error.to_string()),
        }

        match read_file(fragmentPath) {
            Ok(content) => fragmentCode = content,
            Err(error) => return Err(error.to_string()),
        }

        unsafe {
            // vertex shader
            let vertexShader = gl::CreateShader(gl::VERTEX_SHADER);
            let fragmentShader = gl::CreateShader(gl::FRAGMENT_SHADER);
            shader.programId = gl::CreateProgram();

            let c_string = c_string!(vertexCode);
            gl::ShaderSource(vertexShader, 1, &c_string.as_ptr(), ptr::null());
            gl::CompileShader(vertexShader);
            match checkCompileErrors(vertexShader, "VERTEX") {
                Ok(_) => {}
                Err(error) => return Err(error),
            }

            let c_string = c_string!(fragmentCode);
            gl::ShaderSource(fragmentShader, 1, &c_string.as_ptr(), ptr::null());
            gl::CompileShader(fragmentShader);

            match checkCompileErrors(fragmentShader, "FRAGMENT") {
                Ok(_) => {}
                Err(error) => return Err(error),
            }

            // link the first program object
            gl::AttachShader(shader.programId, vertexShader);
            gl::AttachShader(shader.programId, fragmentShader);
            gl::LinkProgram(shader.programId);

            match checkCompileErrors(shader.programId, "PROGRAM") {
                Ok(_) => {}
                Err(error) => return Err(error),
            }

            // Now that the shader programs have been built we can free up memory by deleting the shaders.
            gl::DeleteShader(vertexShader);
            gl::DeleteShader(fragmentShader);
        }

        Ok(shader)
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
            gl::Uniform1i(gl::GetUniformLocation(self.programId, c_string.as_ptr()), v);
        }
    }

    // ------------------------------------------------------------------------
    pub fn setInt(&self, name: &str, value: i32) {
        unsafe {
            let c_string = c_string!(name);
            gl::Uniform1i(gl::GetUniformLocation(self.programId, c_string.as_ptr()), value);
        }
    }

    // ------------------------------------------------------------------------
    pub fn setFloat(&self, name: &str, value: f32) {
        unsafe {
            let c_string = c_string!(name);
            gl::Uniform1f(gl::GetUniformLocation(self.programId, c_string.as_ptr()), value);
        }
    }
}

fn read_file(filename: &str) -> Result<String, Error> {
    let mut content: String = Default::default();
    let mut file = File::open(Path::new(filename))?;
    file.read_to_string(&mut content)?;
    Ok(content)
}

fn checkCompileErrors(shaderId: u32, checkType: &str) -> Result<(), String> {
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
                return Err(String::from_utf8_lossy(&infoLog).to_string());
            }
        } else {
            gl::GetProgramiv(shaderId, gl::LINK_STATUS, &mut status);
            if status != (gl::TRUE as GLint) {
                let mut len = 0;
                gl::GetProgramiv(shaderId, gl::INFO_LOG_LENGTH, &mut len);
                // Subtract 1 to skip the trailing null character.
                let mut infoLog = vec![0; len as usize - 1];
                gl::GetProgramInfoLog(shaderId, 1024, ptr::null_mut(), infoLog.as_mut_ptr() as *mut GLchar);
                let error_msg = String::from_utf8_lossy(&infoLog).to_string();
                return Err(error_msg);
            }
        }
    }
    Ok(())
}
