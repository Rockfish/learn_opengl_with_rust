use std::ffi::c_char;
use std::ffi::CString;
pub mod shader_s;

//
// Conversion helpers
//

// from:  name: &str   to:  *const c_char
//
// example calling gl function:
//
//  GLint foo(name: *const GLchar);
//
// use:
//
//    let name = "thingy";
//    foo(c_str(name));
//
pub fn c_str<T: Into<Vec<u8>>>(t: T) -> *const c_char {
    let c_str = CString::new(t).unwrap();
    return c_str.as_ptr();
}
