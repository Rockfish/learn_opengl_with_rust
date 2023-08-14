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
//    let name = c_str!("thingy");
//    foo(name.as_ptr());
//
#[macro_export]
macro_rules! c_str {
    ($a_string:expr) => {{
        CString::new($a_string).unwrap()
    }};
}
