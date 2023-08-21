#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(unused_assignments)]
#![allow(clippy::zero_ptr)]
#![allow(clippy::assign_op_pattern)]

extern crate glfw;

use glad_gl::gl;
use glad_gl::gl::{GLchar, GLint, GLsizei, GLsizeiptr, GLuint};
use glfw::{Action, Context, Key};
use std::ffi::CString;
use std::{mem, ptr};

const SCR_WIDTH: u32 = 800;
const SCR_HEIGHT: u32 = 800;

const VERTEX_SHADER_SOURCE: &str = r#"#version 330 core
    layout (location = 0) in vec3 aPos;
    void main()
    {
       gl_Position = vec4(aPos.x, aPos.y, aPos.z, 1.0);
    }"#;

const FRAGMENT_SHADER_SOURCE: &str = r#"#version 330 core
    out vec4 FragColor;
    void main()
    {
       FragColor = vec4(1.0f, 0.5f, 0.2f, 1.0f);
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
    let mut VAOs: [GLuint; 2] = [0; 2];
    // Vertex Buffer Object id
    let mut VBOs: [GLuint; 2] = [0; 2];
    // Shader Program id
    let mut shaderProgram: GLuint = 0;

    unsafe {
        // vertex shader
        let vertexShader = gl::CreateShader(gl::VERTEX_SHADER);
        let c_source = CString::new(VERTEX_SHADER_SOURCE).unwrap();

        gl::ShaderSource(vertexShader, 1, &c_source.as_ptr(), ptr::null());
        gl::CompileShader(vertexShader);

        let mut status = gl::FALSE as GLint;
        gl::GetShaderiv(vertexShader, gl::COMPILE_STATUS, &mut status);

        if status != (gl::TRUE as GLint) {
            panic!("Vertex shader compile failed.");
        }

        // fragment shader
        let fragmentShader = gl::CreateShader(gl::FRAGMENT_SHADER);
        let c_source = CString::new(FRAGMENT_SHADER_SOURCE).unwrap();

        gl::ShaderSource(fragmentShader, 1, &c_source.as_ptr(), ptr::null());
        gl::CompileShader(fragmentShader);

        let mut status = gl::FALSE as GLint;
        gl::GetShaderiv(fragmentShader, gl::COMPILE_STATUS, &mut status);

        if status != (gl::TRUE as GLint) {
            panic!("Fragment shader compile failed.");
        }

        // link shaders to the shader program.
        shaderProgram = gl::CreateProgram();
        gl::AttachShader(shaderProgram, vertexShader);
        gl::AttachShader(shaderProgram, fragmentShader);
        gl::LinkProgram(shaderProgram);

        let mut status = gl::FALSE as GLint;
        gl::GetProgramiv(shaderProgram, gl::LINK_STATUS, &mut status);

        if status != (gl::TRUE as GLint) {
            let mut len = 0;
            gl::GetProgramiv(shaderProgram, gl::INFO_LOG_LENGTH, &mut len);
            // Subtract 1 to skip the trailing null character.
            let mut infoLog = vec![0; len as usize - 1];
            gl::GetProgramInfoLog(
                shaderProgram,
                512,
                ptr::null_mut(),
                infoLog.as_mut_ptr() as *mut GLchar,
            );
            panic!("Shader program linking failed: {:?}", infoLog);
        }

        // Now that the shader program has been built we can free up memory by deleting the shaders.
        gl::DeleteShader(vertexShader);
        gl::DeleteShader(fragmentShader);

        // set up vertex data (and buffer(s)) and configure vertex attributes
        // ------------------------------------------------------------------
        // add a new set of vertices to form a second triangle (a total of 6 vertices); the vertex attribute configuration remains the same (still one 3-float position vector per vertex)
        #[rustfmt::skip]
        let firstTriangle: [f32; 9] = [
            -0.9, -0.5, 0.0,  // left
            -0.0, -0.5, 0.0,  // right
            -0.45, 0.5, 0.0,  // top
        ];

        #[rustfmt::skip]
        let secondTriangle: [f32; 9] = [
            0.0, -0.5, 0.0,  // left
            0.9, -0.5, 0.0,  // right
            0.45, 0.5, 0.0   // top
        ];

        // Generate the Vertex Array objects and store their ids.
        // we can also generate multiple VAOs or buffers at the same time
        gl::GenVertexArrays(2, VAOs.as_mut_ptr());

        // Generate the Buffer objects and store their ids.
        gl::GenBuffers(2, VBOs.as_mut_ptr());

        // first triangle setup
        // --------------------
        gl::BindVertexArray(VAOs[0]);

        gl::BindBuffer(gl::ARRAY_BUFFER, VBOs[0]);

        gl::BufferData(
            gl::ARRAY_BUFFER,
            (firstTriangle.len() * mem::size_of::<f32>()) as GLsizeiptr,
            mem::transmute(firstTriangle.as_ptr()),
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

        // second triangle setup
        // ---------------------
        gl::BindVertexArray(VAOs[1]);

        gl::BindBuffer(gl::ARRAY_BUFFER, VBOs[1]);

        gl::BufferData(
            gl::ARRAY_BUFFER,
            (secondTriangle.len() * mem::size_of::<f32>()) as GLsizeiptr,
            mem::transmute(secondTriangle.as_ptr()),
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

        // uncomment this call to draw in wireframe polygons.
        // gl::PolygonMode(gl::FRONT_AND_BACK, gl::LINE);
    }

    while !window.should_close() {
        glfw.poll_events();
        for (_, event) in glfw::flush_messages(&events) {
            handle_window_event(&mut window, event);
        }

        unsafe {
            gl::ClearColor(0.3, 0.3, 0.3, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);

            gl::UseProgram(shaderProgram);

            // draw first triangle using the data from the first VAO
            gl::BindVertexArray(VAOs[0]);
            gl::DrawArrays(gl::TRIANGLES, 0, 3);

            // then we draw the second triangle using the data from the second VAO
            gl::BindVertexArray(VAOs[1]);
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
