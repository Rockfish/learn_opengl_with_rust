extern crate glfw;

use glad_gl::gl;
use glfw::{Action, Context, Key};

fn main() {
    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();

    let (mut window, events) = glfw
        .create_window(600, 600, "Hello this is window", glfw::WindowMode::Windowed)
        .expect("Failed to create GLFW window.");

    window.set_key_polling(true);
    window.make_current();

    // Initialize glad
    gl::load(|e| glfw.get_proc_address_raw(e) as *const std::os::raw::c_void);

    while !window.should_close() {
        glfw.poll_events();
        for (_, event) in glfw::flush_messages(&events) {
            handle_window_event(&mut window, event);
        }

        unsafe {
            // gl::Viewport(0, 0, 300, 300);
            gl::ClearColor(0.7, 0.9, 0.1, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }

        window.swap_buffers();
    }
}

fn handle_window_event(window: &mut glfw::Window, event: glfw::WindowEvent) {
    match event {
        glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => window.set_should_close(true),
        evt => {
            println!("WindowEvent: {:?}", evt);
        }
    }
}
