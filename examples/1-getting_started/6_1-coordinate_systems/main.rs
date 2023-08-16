#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(unused_assignments)]

extern crate glfw;

use glad_gl::gl;
use glad_gl::gl::{GLint, GLsizei, GLsizeiptr, GLuint, GLvoid};
use glam::*;
use glfw::{Action, Context, Key};
use learnopengl_lib::shader_m::Shader_M;
use learnopengl_lib::{c_string, gl_get_uniform_location, size_of_float, size_of_uint};
use std::ffi::CString;
use std::mem;

const SCR_WIDTH: u32 = 800;
const SCR_HEIGHT: u32 = 800;
const SIZE_OF_FLOAT: usize = mem::size_of::<f32>();

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

    // Vertex Array Object id
    let mut VAO: GLuint = 0;
    // Vertex Buffer Object id
    let mut VBO: GLuint = 0;
    // Element Buffer Object id
    let mut EBO: GLuint = 0;
    // Texture ids
    let mut texture1: GLuint = 0;
    let mut texture2: GLuint = 0;
    // Shader program
    let mut ourShader = Shader_M::new();

    unsafe {
        // build and compile our shader program
        // ------------------------------------
        ourShader
            .build(
                "examples/1-getting_started/6_1-coordinate_systems/6_1-coordinate_systems.vert",
                "examples/1-getting_started/6_1-coordinate_systems/6_1-coordinate_systems.frag",
            )
            .unwrap();

        // set up vertex data (and buffer(s)) and configure vertex attributes
        // ------------------------------------------------------------------
        #[rustfmt::skip]
        let vertices: [f32; 20] = [
            // positions      // texture coordinates
            0.5,  0.5, 0.0,   1.0, 1.0, // top right
            0.5, -0.5, 0.0,   1.0, 0.0, // bottom right
           -0.5, -0.5, 0.0,   0.0, 0.0, // bottom left
           -0.5,  0.5, 0.0,   0.0, 1.0  // top left
        ];

        #[rustfmt::skip]
        let indices: [u32; 6] = [
            0, 1, 3, // first triangle
            1, 2, 3  // second triangle
        ];

        gl::GenVertexArrays(1, &mut VAO);
        gl::GenBuffers(1, &mut VBO);
        gl::GenBuffers(1, &mut EBO);

        gl::BindVertexArray(VAO);

        gl::BindBuffer(gl::ARRAY_BUFFER, VBO);
        gl::BufferData(
            gl::ARRAY_BUFFER,
            size_of_float!(vertices.len()) as GLsizeiptr,
            vertices.as_ptr() as *const GLvoid,
            gl::STATIC_DRAW,
        );

        gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, EBO);
        gl::BufferData(
            gl::ELEMENT_ARRAY_BUFFER,
            size_of_uint!(indices.len()) as GLsizeiptr,
            indices.as_ptr() as *const GLvoid,
            gl::STATIC_DRAW,
        );

        // position attribute
        gl::VertexAttribPointer(
            0,
            3,
            gl::FLOAT,
            gl::FALSE,
            5 * SIZE_OF_FLOAT as GLsizei,
            0 as *const GLvoid,
        );
        gl::EnableVertexAttribArray(0);

        // texture coordinate attribute
        gl::VertexAttribPointer(
            1,
            2,
            gl::FLOAT,
            gl::FALSE,
            (5 * SIZE_OF_FLOAT) as GLsizei,
            (3 * SIZE_OF_FLOAT) as *const GLvoid,
        );
        gl::EnableVertexAttribArray(1);

        // load and create a texture
        // -------------------------
        // texture 1
        // -------------------------
        gl::GenTextures(1, &mut texture1);
        // all upcoming GL_TEXTURE_2D operations now have effect on this texture object
        gl::BindTexture(gl::TEXTURE_2D, texture1);
        // set the texture wrapping parameters
        // set texture wrapping to gl::REPEAT (default wrapping method)
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT as GLint);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT as GLint);

        // set texture filtering parameters
        gl::TexParameteri(
            gl::TEXTURE_2D,
            gl::TEXTURE_MIN_FILTER,
            gl::LINEAR_MIPMAP_LINEAR as GLint,
        );
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as GLint);

        // load image, create texture and generate mipmaps
        let img = image::open("resources/textures/container.jpg").expect("Texture failed to load");
        let (width, height) = (img.width() as GLsizei, img.height() as GLsizei);
        let data = img.into_rgb8().into_raw();

        gl::TexImage2D(
            gl::TEXTURE_2D,
            0,
            gl::RGB as GLint,
            width,
            height,
            0,
            gl::RGB,
            gl::UNSIGNED_BYTE,
            data.as_ptr() as *const GLvoid,
        );
        gl::GenerateMipmap(gl::TEXTURE_2D);

        // texture 2
        // -------------------------
        gl::GenTextures(1, &mut texture2);
        // all upcoming GL_TEXTURE_2D operations now have effect on this texture object
        gl::BindTexture(gl::TEXTURE_2D, texture2);
        // set the texture wrapping parameters
        // set texture wrapping to gl::REPEAT (default wrapping method)
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT as GLint);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT as GLint);

        // set texture filtering parameters
        gl::TexParameteri(
            gl::TEXTURE_2D,
            gl::TEXTURE_MIN_FILTER,
            gl::LINEAR_MIPMAP_LINEAR as GLint,
        );
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as GLint);

        // load image, create texture and generate mipmaps
        let img =
            image::open("resources/textures/awesomeface.png").expect("Texture failed to load");
        let (width, height) = (img.width() as GLsizei, img.height() as GLsizei);

        // flip image vertically so that the texture is rendered upright
        // use into_rgba since the image has an alpha transparency
        let img_data = img.flipv().into_rgba8().into_raw();

        gl::TexImage2D(
            gl::TEXTURE_2D,
            0,
            gl::RGB as GLint,
            width,
            height,
            0,
            gl::RGBA, // RGB with Alpha
            gl::UNSIGNED_BYTE,
            img_data.as_ptr() as *const GLvoid,
        );
        gl::GenerateMipmap(gl::TEXTURE_2D);

        // tell opengl for each sampler to which texture unit it belongs to (only has to be done once)
        // -------------------------------------------------------------------------------------------
        ourShader.use_shader();
        ourShader.setInt("texture1", 0);
        ourShader.setInt("texture2", 1);
    }

    // render loop
    while !window.should_close() {
        glfw.poll_events();
        for (_, event) in glfw::flush_messages(&events) {
            handle_window_event(&mut window, event);
        }

        unsafe {
            // render
            gl::ClearColor(0.2, 0.3, 0.3, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);

            // bind textures on corresponding texture units
            gl::ActiveTexture(gl::TEXTURE0);
            gl::BindTexture(gl::TEXTURE_2D, texture1);
            gl::ActiveTexture(gl::TEXTURE1);
            gl::BindTexture(gl::TEXTURE_2D, texture2);

            // activate shader
            ourShader.use_shader();

            // create transformations using glam
            let mut model = Mat4::IDENTITY;
            let mut view = Mat4::IDENTITY;
            let mut projection = Mat4::IDENTITY;

            model = model * Mat4::from_rotation_x((-55.0f32).to_radians());
            view = view * Mat4::from_translation(Vec3::new(0.0, 0.0, -3.0));
            projection = projection
                * Mat4::perspective_rh_gl(
                    45.0f32.to_radians(),
                    (SCR_WIDTH / SCR_HEIGHT) as f32,
                    0.1,
                    100.0,
                );

            // retrieve the matrix uniform locations
            let modelLoc = gl_get_uniform_location!(ourShader.programId, "model");
            let viewLoc = gl_get_uniform_location!(ourShader.programId, "view");

            // pass them to the shaders two different ways
            gl::UniformMatrix4fv(modelLoc, 1, gl::FALSE, model.to_cols_array().as_ptr());
            gl::UniformMatrix4fv(viewLoc, 1, gl::FALSE, view.to_cols_array().as_ptr());
            // note: currently we set the projection matrix each frame, but since the projection matrix rarely changes
            // it's often best practice to set it outside the main loop only once.
            ourShader.setMat4("projection", &projection);

            // render container
            gl::BindVertexArray(VAO);
            gl::DrawElements(gl::TRIANGLES, 6, gl::UNSIGNED_INT, 0 as *const GLvoid);
        }

        window.swap_buffers();
    }

    // optional: de-allocate all resources once they've outlived their purpose:
    // ------------------------------------------------------------------------
    unsafe {
        gl::DeleteVertexArrays(2, &VAO);
        gl::DeleteBuffers(2, &VBO);
        gl::DeleteBuffers(1, &EBO);
        gl::DeleteProgram(ourShader.programId);
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
