#![feature(is_sorted)]
#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(unused_assignments)]
#![allow(clippy::zero_ptr)]
#![allow(clippy::assign_op_pattern)]

extern crate glfw;

use glad_gl::gl;
use glad_gl::gl::{GLenum, GLint, GLsizei, GLsizeiptr, GLuint, GLvoid};
use glam::{vec3, Mat4};
use glfw::{Action, Context, Key};
use learn_opengl_with_rust::camera::{Camera, CameraMovement};
use learn_opengl_with_rust::shader::Shader;
use learn_opengl_with_rust::{size_of_floats, SIZE_OF_FLOAT};
use std::{mem, ptr};

const SCR_WIDTH: f32 = 800.0;
const SCR_HEIGHT: f32 = 800.0;

struct State {
    camera: Camera,
    deltaTime: f32,
    lastFrame: f32,
    firstMouse: bool,
    lastX: f32,
    lastY: f32,
}

fn main() {
    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();

    glfw.window_hint(glfw::WindowHint::ContextVersion(3, 3));
    glfw.window_hint(glfw::WindowHint::OpenGlProfile(glfw::OpenGlProfileHint::Core));

    // for Apple
    glfw.window_hint(glfw::WindowHint::OpenGlForwardCompat(true));

    // Use a multisample buffer with 4 samples
    glfw.window_hint(glfw::WindowHint::Samples(Some(4)));

    let (mut window, events) = glfw
        .create_window(SCR_WIDTH as u32, SCR_HEIGHT as u32, "LearnOpenGL", glfw::WindowMode::Windowed)
        .expect("Failed to create GLFW window.");

    // Turn on all GLFW polling so that we can receive all WindowEvents
    window.set_all_polling(true);
    window.make_current();

    // Initialize glad: load all OpenGL function pointers
    // --------------------------------------------------
    gl::load(|e| glfw.get_proc_address_raw(e) as *const std::os::raw::c_void);

    let camera = Camera::camera_vec3(vec3(0.0, 0.0, 3.0));

    // Initialize the world state
    let mut state = State {
        camera,
        deltaTime: 0.0,
        lastFrame: 0.0,
        firstMouse: true,
        lastX: SCR_WIDTH / 2.0,
        lastY: SCR_HEIGHT / 2.0,
    };

    unsafe {
        // configure global opengl state
        // -----------------------------
        gl::Enable(gl::DEPTH_TEST);
        // enabled by default on some drivers, but not all so always enable to make sure
        gl::Enable(gl::MULTISAMPLE);
    }

    // build and compile our shader program
    // ------------------------------------
    let shader = Shader::new(
        "examples/4-advanced_opengl/11_2-anti_aliasing_offscreen/11_2-anti_aliasing.vert",
        "examples/4-advanced_opengl/11_2-anti_aliasing_offscreen/11_2-anti_aliasing.frag",
        None,
    )
    .unwrap();

    let screenShader = Shader::new(
        "examples/4-advanced_opengl/11_2-anti_aliasing_offscreen/11_2-aa_post.vert",
        "examples/4-advanced_opengl/11_2-anti_aliasing_offscreen/11_2-aa_post.frag",
        None,
    )
    .unwrap();

    // set up vertex data (and buffer(s)) and configure vertex attributes
    // ------------------------------------------------------------------
    #[rustfmt::skip]
    let cubeVertices: [f32; 108] = [
        // positions
        -0.5, -0.5, -0.5,
         0.5, -0.5, -0.5,
         0.5,  0.5, -0.5,
         0.5,  0.5, -0.5,
        -0.5,  0.5, -0.5,
        -0.5, -0.5, -0.5,

        -0.5, -0.5,  0.5,
         0.5, -0.5,  0.5,
         0.5,  0.5,  0.5,
         0.5,  0.5,  0.5,
        -0.5,  0.5,  0.5,
        -0.5, -0.5,  0.5,

        -0.5,  0.5,  0.5,
        -0.5,  0.5, -0.5,
        -0.5, -0.5, -0.5,
        -0.5, -0.5, -0.5,
        -0.5, -0.5,  0.5,
        -0.5,  0.5,  0.5,

         0.5,  0.5,  0.5,
         0.5,  0.5, -0.5,
         0.5, -0.5, -0.5,
         0.5, -0.5, -0.5,
         0.5, -0.5,  0.5,
         0.5,  0.5,  0.5,

        -0.5, -0.5, -0.5,
         0.5, -0.5, -0.5,
         0.5, -0.5,  0.5,
         0.5, -0.5,  0.5,
        -0.5, -0.5,  0.5,
        -0.5, -0.5, -0.5,

        -0.5,  0.5, -0.5,
         0.5,  0.5, -0.5,
         0.5,  0.5,  0.5,
         0.5,  0.5,  0.5,
        -0.5,  0.5,  0.5,
        -0.5,  0.5, -0.5
    ];

    #[rustfmt::skip]
    let quadVertices: [f32; 24] = [
        // vertex attributes for a quad that fills the entire screen in Normalized Device Coordinates.
        // positions   // texCoords
        -1.0,  1.0,  0.0, 1.0,
        -1.0, -1.0,  0.0, 0.0,
         1.0, -1.0,  1.0, 0.0,

        -1.0,  1.0,  0.0, 1.0,
         1.0, -1.0,  1.0, 0.0,
         1.0,  1.0,  1.0, 1.0
    ];

    // Vertex Array Object id
    let mut cubeVAO: GLuint = 0;
    let mut cubeVBO: GLuint = 0;
    let mut quadVAO: GLuint = 0;
    let mut quadVBO: GLuint = 0;
    let mut framebuffer: GLuint = 0;
    let mut textureColorBufferMultiSampled: GLuint = 0;
    let mut rbo: GLuint = 0;
    let mut intermediateFBO: GLuint = 0;
    let mut screenTexture: GLuint = 0;

    unsafe {
        // setup cube VAO
        gl::GenVertexArrays(1, &mut cubeVAO);
        gl::GenBuffers(1, &mut cubeVBO);
        gl::BindVertexArray(cubeVAO);
        gl::BindBuffer(gl::ARRAY_BUFFER, cubeVBO);
        gl::BufferData(
            gl::ARRAY_BUFFER,
            size_of_floats!(cubeVertices.len()) as GLsizeiptr,
            cubeVertices.as_ptr() as *const GLvoid,
            gl::STATIC_DRAW,
        );
        gl::EnableVertexAttribArray(0);
        gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, size_of_floats!(3) as GLsizei, 0 as *const GLvoid);

        // setup screen VAO
        gl::GenVertexArrays(1, &mut quadVAO);
        gl::GenBuffers(1, &mut quadVBO);
        gl::BindVertexArray(quadVAO);
        gl::BindBuffer(gl::ARRAY_BUFFER, quadVAO);
        gl::BufferData(
            gl::ARRAY_BUFFER,
            size_of_floats!(quadVertices.len()) as GLsizeiptr,
            quadVertices.as_ptr() as *const GLvoid,
            gl::STATIC_DRAW,
        );
        gl::EnableVertexAttribArray(0);
        gl::VertexAttribPointer(0, 2, gl::FLOAT, gl::FALSE, size_of_floats!(4) as GLsizei, 0 as *const GLvoid);
        gl::EnableVertexAttribArray(1);
        gl::VertexAttribPointer(
            1,
            2,
            gl::FLOAT,
            gl::FALSE,
            size_of_floats!(4) as GLsizei,
            (2 * SIZE_OF_FLOAT) as *const GLvoid,
        );

        // configure MSAA framebuffer
        // --------------------------
        gl::GenFramebuffers(1, &mut framebuffer);
        gl::BindFramebuffer(gl::FRAMEBUFFER, framebuffer);
        // create a multisampled color attachment texture
        gl::GenTextures(1, &mut textureColorBufferMultiSampled);
        gl::BindTexture(gl::TEXTURE_2D_MULTISAMPLE, textureColorBufferMultiSampled);

        gl::TexImage2DMultisample(
            gl::TEXTURE_2D_MULTISAMPLE,
            4,
            gl::RGB as GLenum,
            SCR_WIDTH as GLsizei,
            SCR_HEIGHT as GLsizei,
            gl::TRUE,
        );
        gl::BindTexture(gl::TEXTURE_2D_MULTISAMPLE, 0);
        gl::FramebufferTexture2D(
            gl::FRAMEBUFFER,
            gl::COLOR_ATTACHMENT0,
            gl::TEXTURE_2D_MULTISAMPLE,
            textureColorBufferMultiSampled,
            0,
        );

        // create a (also multisampled) renderbuffer object for depth and stencil attachments
        gl::GenRenderbuffers(1, &mut rbo);
        gl::BindRenderbuffer(gl::RENDERBUFFER, rbo);
        gl::RenderbufferStorageMultisample(gl::RENDERBUFFER, 4, gl::DEPTH24_STENCIL8, SCR_WIDTH as GLsizei, SCR_HEIGHT as GLsizei);
        gl::FramebufferRenderbuffer(gl::FRAMEBUFFER, gl::DEPTH_STENCIL_ATTACHMENT, gl::RENDERBUFFER, rbo);

        // now that we actually created the framebuffer and added all attachments we want to check if it is actually complete now
        if gl::CheckFramebufferStatus(gl::FRAMEBUFFER) != gl::FRAMEBUFFER_COMPLETE {
            println!("ERROR::FRAMEBUFFER:: Framebuffer is not complete!");
        }
        gl::BindFramebuffer(gl::FRAMEBUFFER, 0);

        // configure second post-processing framebuffer
        gl::GenFramebuffers(1, &mut intermediateFBO);
        gl::BindFramebuffer(gl::FRAMEBUFFER, intermediateFBO);

        // create a color attachment texture
        gl::GenTextures(1, &mut screenTexture);
        gl::BindTexture(gl::TEXTURE_2D, screenTexture);
        gl::TexImage2D(
            gl::TEXTURE_2D,
            0,
            gl::RGB as GLint,
            SCR_WIDTH as GLsizei,
            SCR_HEIGHT as GLsizei,
            0,
            gl::RGB,
            gl::UNSIGNED_BYTE,
            ptr::null(),
        );
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as GLint);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as GLint);
        gl::FramebufferTexture2D(gl::FRAMEBUFFER, gl::COLOR_ATTACHMENT0, gl::TEXTURE_2D, screenTexture, 0);

        if gl::CheckFramebufferStatus(gl::FRAMEBUFFER) != gl::FRAMEBUFFER_COMPLETE {
            println!("ERROR::FRAMEBUFFER:: Framebuffer is not complete!");
        }
        gl::BindFramebuffer(gl::FRAMEBUFFER, 0);
    }

    screenShader.use_shader();
    screenShader.set_int("screenTexture", 0);

    // render loop
    while !window.should_close() {
        let currentFrame = glfw.get_time() as f32;
        state.deltaTime = currentFrame - state.lastFrame;
        state.lastFrame = currentFrame;

        glfw.poll_events();
        for (_, event) in glfw::flush_messages(&events) {
            handle_window_event(&mut window, event, &mut state);
        }

        unsafe {
            // render
            gl::ClearColor(0.1, 0.1, 0.1, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);

            // 1. draw scene as normal in multisampled buffers
            gl::BindFramebuffer(gl::FRAMEBUFFER, framebuffer);
            gl::ClearColor(0.1, 0.1, 0.1, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
            gl::Enable(gl::DEPTH_TEST);

            // set transformation matrices
            shader.use_shader();
            let projection = Mat4::perspective_rh_gl(state.camera.Zoom.to_radians(), SCR_WIDTH / SCR_HEIGHT, 0.1, 1000.0);
            let view = state.camera.GetViewMatrix();
            shader.set_mat4("projection", &projection);
            shader.set_mat4("view", &view);
            shader.set_mat4("model", &Mat4::IDENTITY);

            gl::BindVertexArray(cubeVAO);
            gl::DrawArrays(gl::TRIANGLES, 0, 36);

            // 2. now blit multisampled buffer(s) to normal colorbuffer of intermediate FBO. Image is stored in screenTexture
            gl::BindFramebuffer(gl::READ_FRAMEBUFFER, framebuffer);
            gl::BindFramebuffer(gl::DRAW_FRAMEBUFFER, intermediateFBO);
            gl::BlitFramebuffer(
                0,
                0,
                SCR_WIDTH as GLint,
                SCR_HEIGHT as GLint,
                0,
                0,
                SCR_WIDTH as GLint,
                SCR_HEIGHT as GLint,
                gl::COLOR_BUFFER_BIT,
                gl::NEAREST,
            );

            // 3. now render quad with scene's visuals as its texture image
            gl::BindFramebuffer(gl::FRAMEBUFFER, 0);
            gl::ClearColor(1.0, 1.0, 1.0, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
            gl::Disable(gl::DEPTH_TEST);

            // draw Screen quad
            screenShader.use_shader();
            gl::BindVertexArray(quadVAO);
            gl::ActiveTexture(gl::TEXTURE0);
            gl::BindTexture(gl::TEXTURE_2D, screenTexture); // use the now resolved color attachment as the quad's texture
            gl::DrawArrays(gl::TRIANGLES, 0, 6);
        }

        window.swap_buffers();
    }

    // optional: de-allocate all resources once they've outlived their purpose:
    // ------------------------------------------------------------------------
    unsafe {
        gl::DeleteShader(shader.id);
    }
}

//
// GLFW maps callbacks to events.
//
fn handle_window_event(window: &mut glfw::Window, event: glfw::WindowEvent, state: &mut State) {
    match event {
        glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => window.set_should_close(true),
        glfw::WindowEvent::FramebufferSize(width, height) => {
            framebuffer_size_event(window, width, height);
        }
        glfw::WindowEvent::Key(Key::W, _, _, _) => {
            state.camera.ProcessKeyboard(CameraMovement::FORWARD, state.deltaTime);
        }
        glfw::WindowEvent::Key(Key::S, _, _, _) => {
            state.camera.ProcessKeyboard(CameraMovement::BACKWARD, state.deltaTime);
        }
        glfw::WindowEvent::Key(Key::A, _, _, _) => {
            state.camera.ProcessKeyboard(CameraMovement::LEFT, state.deltaTime);
        }
        glfw::WindowEvent::Key(Key::D, _, _, _) => {
            state.camera.ProcessKeyboard(CameraMovement::RIGHT, state.deltaTime);
        }
        glfw::WindowEvent::Key(Key::Q, _, _, _) => {
            state.camera.ProcessKeyboard(CameraMovement::UP, state.deltaTime);
        }
        glfw::WindowEvent::Key(Key::Z, _, _, _) => {
            state.camera.ProcessKeyboard(CameraMovement::DOWN, state.deltaTime);
        }
        glfw::WindowEvent::CursorPos(xpos, ypos) => mouse_handler(state, xpos, ypos),
        glfw::WindowEvent::Scroll(xoffset, ysoffset) => scroll_handler(state, xoffset, ysoffset),
        _evt => {
            // println!("WindowEvent: {:?}", evt);
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

fn mouse_handler(state: &mut State, xposIn: f64, yposIn: f64) {
    let xpos = xposIn as f32;
    let ypos = yposIn as f32;

    if state.firstMouse {
        state.lastX = xpos;
        state.lastY = ypos;
        state.firstMouse = false;
    }

    let xoffset = xpos - state.lastX;
    let yoffset = state.lastY - ypos; // reversed since y-coordinates go from bottom to top

    state.lastX = xpos;
    state.lastY = ypos;

    state.camera.ProcessMouseMovement(xoffset, yoffset, true);
}

fn scroll_handler(state: &mut State, _xoffset: f64, yoffset: f64) {
    state.camera.ProcessMouseScroll(yoffset as f32);
}
