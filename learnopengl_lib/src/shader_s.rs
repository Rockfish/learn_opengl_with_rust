#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(unused_assignments)]

use glad_gl::gl;
use glad_gl::gl::{GLchar, GLint};

use std::fs::File;
use std::io::prelude::*;
use std::io::Error;
use std::path::Path;
use std::ptr;

use crate::*;

type ID = u32;

pub struct Shader_S {
    pub shaderProgramId: ID,
}

impl Shader_S {
    pub fn new() -> Shader_S {
        Shader_S { shaderProgramId: 0 }
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
            self.shaderProgramId = gl::CreateProgram();

            let c_source = CString::new(vertexCode).unwrap();
            gl::ShaderSource(vertexShader, 1, &c_source.as_ptr(), ptr::null());
            gl::CompileShader(vertexShader);
            checkCompileErrors(vertexShader, "VERTEX");

            let c_source = CString::new(fragmentCode).unwrap();
            gl::ShaderSource(fragmentShader, 1, &c_source.as_ptr(), ptr::null());
            gl::CompileShader(fragmentShader);
            checkCompileErrors(fragmentShader, "VERTEX");

            // link the first program object
            gl::AttachShader(self.shaderProgramId, vertexShader);
            gl::AttachShader(self.shaderProgramId, fragmentShader);
            gl::LinkProgram(self.shaderProgramId);
            checkCompileErrors(self.shaderProgramId, "PROGRAM");

            // Now that the shader programs have been built we can free up memory by deleting the shaders.
            gl::DeleteShader(vertexShader);
            gl::DeleteShader(fragmentShader);
        }

        Ok(self.shaderProgramId)
    }

    pub fn use_shader(&self) {
        unsafe {
            gl::UseProgram(self.shaderProgramId);
        }
    }

    // utility uniform functions
    // ------------------------------------------------------------------------
    pub fn setBool(&self, name: &str, value: bool) {
        unsafe {
            let v = if value { 1 } else { 0 };
            gl::Uniform1i(gl::GetUniformLocation(self.shaderProgramId, c_str(name)), v);
        }
    }

    // ------------------------------------------------------------------------
    pub fn setInt(&self, name: &str, value: i32) {
        unsafe {
            gl::Uniform1i(
                gl::GetUniformLocation(self.shaderProgramId, c_str(name)),
                value,
            );
        }
    }

    // ------------------------------------------------------------------------
    pub fn setFloat(&self, name: &str, value: f32) {
        unsafe {
            gl::Uniform1f(
                gl::GetUniformLocation(self.shaderProgramId, c_str(name)),
                value,
            );
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
                gl::GetProgramInfoLog(
                    shaderId,
                    1024,
                    ptr::null_mut(),
                    infoLog.as_mut_ptr() as *mut GLchar,
                );
                panic!(
                    "Shader compilation failed.\n{}",
                    String::from_utf8_lossy(&infoLog)
                );
            }
        } else {
            gl::GetProgramiv(shaderId, gl::LINK_STATUS, &mut status);
            if status != (gl::TRUE as GLint) {
                let mut len = 0;
                gl::GetProgramiv(shaderId, gl::INFO_LOG_LENGTH, &mut len);
                // Subtract 1 to skip the trailing null character.
                let mut infoLog = vec![0; len as usize - 1];
                gl::GetProgramInfoLog(
                    shaderId,
                    1024,
                    ptr::null_mut(),
                    infoLog.as_mut_ptr() as *mut GLchar,
                );
                panic!(
                    "Shader program linking failed.\n{}",
                    String::from_utf8_lossy(&infoLog)
                );
            }
        }
    }
}
