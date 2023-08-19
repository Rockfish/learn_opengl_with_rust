use std::ffi::CString;
use std::mem;

pub mod camera;
pub mod macros;
pub mod shader_m;
pub mod shader_s;

pub const SIZE_OF_FLOAT: usize = mem::size_of::<f32>();
