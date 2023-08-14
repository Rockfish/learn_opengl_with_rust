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
//    let c_str = c_string!("thingy");
//    foo(c_str.as_ptr());
//
#[macro_export]
macro_rules! c_string {
    ($a_string:expr) => {{
        CString::new($a_string).unwrap()
    }};
}

#[macro_export]
macro_rules! size_of_float {
    ($value:expr) => {{
        ($value * mem::size_of::<f32>())
    }};
}

#[macro_export]
macro_rules! size_of_uint {
    ($value:expr) => {{
        ($value * mem::size_of::<u32>())
    }};
}
