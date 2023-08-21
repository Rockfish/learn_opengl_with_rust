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
    let mut VAO: GLuint = 0;
    // Vertex Buffer Object id
    let mut VBO: GLuint = 0;
    // Element Buffer Object id
    let mut EBO: GLuint = 0;
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
        #[rustfmt::skip]
        let vertices: [f32; 12] = [
            0.5,  0.5, 0.0,  // top right
            0.5, -0.5, 0.0,  // bottom right
           -0.5, -0.5, 0.0,  // bottom left
           -0.5,  0.5, 0.0   // top left
        ];

        #[rustfmt::skip]
        let indices: [u32; 6] = [  // note that we start from 0!
            0, 1, 3,  // first Triangle
            1, 2, 3   // second Triangle
        ];

        // Generate the Vertex Array object and store the id.
        gl::GenVertexArrays(1, &mut VAO);

        // Generate the Vertex Buffer and Element objects and store their ids.
        gl::GenBuffers(1, &mut VBO);
        gl::GenBuffers(1, &mut EBO);

        // bind the Vertex Array Object first, then bind and set vertex buffer(s),
        // and then configure vertex attributes(s).
        gl::BindVertexArray(VAO);

        gl::BindBuffer(gl::ARRAY_BUFFER, VBO);

        gl::BufferData(
            gl::ARRAY_BUFFER,
            vertices.len() as GLsizeiptr * mem::size_of::<f32>() as GLsizeiptr,
            mem::transmute(vertices.as_ptr()),
            gl::STATIC_DRAW,
        );

        gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, EBO);

        gl::BufferData(
            gl::ELEMENT_ARRAY_BUFFER,
            indices.len() as GLsizeiptr * mem::size_of::<u32>() as GLsizeiptr,
            mem::transmute(indices.as_ptr()),
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

        // note that this is allowed, the call to glVertexAttribPointer registered VBO as the
        // vertex attribute's bound vertex buffer object so afterwards we can safely unbind
        gl::BindBuffer(gl::ARRAY_BUFFER, 0);

        // remember: do NOT unbind the EBO while a VAO is active as the bound element buffer object
        // IS stored in the VAO; keep the EBO bound.
        //glBindBuffer(GL_ELEMENT_ARRAY_BUFFER, 0);

        // You can unbind the VAO afterwards so other VAO calls won't accidentally modify this VAO,
        // but this rarely happens. Modifying other VAOs requires a call to glBindVertexArray anyways
        // so we generally don't unbind VAOs (nor VBOs) when it's not directly necessary.
        gl::BindVertexArray(0);

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

            // draw our first triangle
            gl::UseProgram(shaderProgram);
            // Bind the VAO we want to use. Seeing as we only have a single VAO there's no need
            // to bind it every time, but we'll do so to keep things a bit more organized.
            gl::BindVertexArray(VAO);
            // gl::DrawArrays(gl::TRIANGLES, 0, 3);
            gl::DrawElements(gl::TRIANGLES, 6, gl::UNSIGNED_INT, ptr::null());
        }

        window.swap_buffers();
    }

    // optional: de-allocate all resources once they've outlived their purpose:
    // ------------------------------------------------------------------------
    unsafe {
        gl::DeleteVertexArrays(1, &VAO);
        gl::DeleteBuffers(1, &VBO);
        gl::DeleteBuffers(1, &EBO);
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
