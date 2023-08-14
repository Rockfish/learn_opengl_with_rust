#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(unused_assignments)]

extern crate glfw;

use glad_gl::gl;
use glad_gl::gl::{GLsizei, GLsizeiptr, GLuint};
use glfw::{Action, Context, Key};
use std::ffi::CString;
use std::{mem, ptr};

const SCR_WIDTH: u32 = 800;
const SCR_HEIGHT: u32 = 800;

const VERTEX_SHADER_SOURCE: &str = r#"#version 330 core
    layout (location = 0) in vec3 aPos;
    void main()
    {
       gl_Position = vec4(aPos, 1.0);
    }"#;

const FRAGMENT_SHADER_1_SOURCE: &str = r#"#version 330 core
    out vec4 FragColor;
    uniform vec4 ourColor;
    void main()
    {
       FragColor = ourColor;
    }"#;

fn main() {
    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();

    glfw.window_hint(glfw::WindowHint::ContextVersion(3, 3));
    glfw.window_hint(glfw::WindowHint::OpenGlProfile(
        glfw::OpenGlProfileHint::Core,
    ));

    // for Apple
    glfw.window_hint(glfw::WindowHint::OpenGlForwardCompat(true));

    let (mut window, events) = glfw
        .create_window(
            SCR_WIDTH,
            SCR_HEIGHT,
            "LearnOpenGL",
            glfw::WindowMode::Windowed,
        )
        .expect("Failed to create GLFW window.");

    // Turn on all GLFW polling so that we can receive all WindowEvents
    window.set_all_polling(true);
    window.make_current();

    // Initialize glad: load all OpenGL function pointers
    // --------------------------------------------------
    gl::load(|e| glfw.get_proc_address_raw(e) as *const std::os::raw::c_void);

    // build and compile our shader program
    // ------------------------------------

    // Vertex Array Object id
    let mut VAOs: [GLuint; 1] = [0; 1];
    // Vertex Buffer Object id
    let mut VBOs: [GLuint; 1] = [0; 1];
    // Shader Program id
    let mut shaderProgram: GLuint = 0;

    unsafe {
        // vertex shader
        let vertexShader = gl::CreateShader(gl::VERTEX_SHADER);
        let fragmentShader = gl::CreateShader(gl::FRAGMENT_SHADER);
        shaderProgram = gl::CreateProgram();

        let c_source = CString::new(VERTEX_SHADER_SOURCE).unwrap();
        gl::ShaderSource(vertexShader, 1, &c_source.as_ptr(), ptr::null());
        gl::CompileShader(vertexShader);

        let c_source = CString::new(FRAGMENT_SHADER_1_SOURCE).unwrap();
        gl::ShaderSource(fragmentShader, 1, &c_source.as_ptr(), ptr::null());
        gl::CompileShader(fragmentShader);

        // link the first program object
        gl::AttachShader(shaderProgram, vertexShader);
        gl::AttachShader(shaderProgram, fragmentShader);
        gl::LinkProgram(shaderProgram);

        // Now that the shader programs have been built we can free up memory by deleting the shaders.
        gl::DeleteShader(vertexShader);
        gl::DeleteShader(fragmentShader);

        // set up vertex data (and buffer(s)) and configure vertex attributes
        // ------------------------------------------------------------------
        #[rustfmt::skip]
        let vertices: [f32; 9] = [
             0.5, -0.5, 0.0,  // bottom right
            -0.5, -0.5, 0.0,  // bottom left
             0.0,  0.5, 0.0,  // top
        ];

        // Generate the Vertex Array objects and store their ids.
        // we can also generate multiple VAOs or buffers at the same time
        gl::GenVertexArrays(1, VAOs.as_mut_ptr());

        // Generate the Buffer objects and store their ids.
        gl::GenBuffers(1, VBOs.as_mut_ptr());

        // triangle setup
        // --------------------
        gl::BindVertexArray(VAOs[0]);

        gl::BindBuffer(gl::ARRAY_BUFFER, VBOs[0]);

        gl::BufferData(
            gl::ARRAY_BUFFER,
            (vertices.len() * mem::size_of::<f32>()) as GLsizeiptr,
            mem::transmute(vertices.as_ptr()),
            gl::STATIC_DRAW,
        );

        gl::VertexAttribPointer(
            0,
            3,
            gl::FLOAT,
            gl::FALSE,
            (3 * mem::size_of::<f32>()) as GLsizei,
            ptr::null(),
        );

        gl::EnableVertexAttribArray(0);

        // You can unbind the VAO afterwards so other VAO calls won't accidentally modify this VAO, but this rarely happens. Modifying other
        // VAOs requires a call to glBindVertexArray anyways so we generally don't unbind VAOs (nor VBOs) when it's not directly necessary.
        // glBindVertexArray(0);

        // bind the VAO (it was already bound, but just to demonstrate): seeing as we only have a single VAO we can
        // just bind it beforehand before rendering the respective triangle; this is another approach.
        gl::BindVertexArray(VAOs[0]);
    }

    while !window.should_close() {
        glfw.poll_events();
        for (_, event) in glfw::flush_messages(&events) {
            handle_window_event(&mut window, event);
        }

        unsafe {
            gl::ClearColor(0.3, 0.3, 0.3, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);

            // be sure to activate the shader before any calls to glUniform
            gl::UseProgram(shaderProgram);

            // update shader uniform
            let timeValue = glfw.get_time();
            let greenValue = (timeValue.sin() / 2.0 + 0.5) as f32;
            let c_str = CString::new("ourColor").unwrap();
            let vertexColorLocation = gl::GetUniformLocation(shaderProgram, c_str.as_ptr());
            gl::Uniform4f(vertexColorLocation, 0.0, greenValue, 0.0, 1.0);

            // render the triangle
            gl::DrawArrays(gl::TRIANGLES, 0, 3);
        }

        window.swap_buffers();
    }

    // optional: de-allocate all resources once they've outlived their purpose:
    // ------------------------------------------------------------------------
    unsafe {
        gl::DeleteVertexArrays(2, VAOs.as_ptr());
        gl::DeleteBuffers(2, VBOs.as_ptr());
        gl::DeleteProgram(shaderProgram);
    }
}

//
// GLFW maps callbacks to events.
//
fn handle_window_event(window: &mut glfw::Window, event: glfw::WindowEvent) {
    match event {
        glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => window.set_should_close(true),
        glfw::WindowEvent::FramebufferSize(width, height) => {
            framebuffer_size_event(window, width, height);
        }
        evt => {
            println!("WindowEvent: {:?}", evt);
        }
    }
}

// glfw: whenever the window size changed (by OS or user resize) this event fires.
// ---------------------------------------------------------------------------------------------
fn framebuffer_size_event(_window: &mut glfw::Window, width: i32, height: i32) {
    // make sure the viewport matches the new window dimensions; note that width and
    // height will be significantly larger than specified on retina displays.
    println!("Framebuffer size: {}, {}", width, height);
    unsafe {
        gl::Viewport(0, 0, width, height);
    }
}
