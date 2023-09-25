#![feature(offset_of)]

pub mod aiscene;
pub mod camera;
pub mod macros;
pub mod mesh;
pub mod model;
pub mod shader;
pub mod shader_m;
pub mod shader_s;

pub const SIZE_OF_FLOAT: usize = std::mem::size_of::<f32>();
