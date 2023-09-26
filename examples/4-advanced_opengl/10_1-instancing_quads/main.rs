#![feature(is_sorted)]
#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(unused_assignments)]
#![allow(clippy::zero_ptr)]
#![allow(clippy::assign_op_pattern)]

extern crate glfw;

use glad_gl::gl;
use glad_gl::gl::{GLsizei, GLsizeiptr, GLuint, GLvoid};
use glam::{vec3, Vec2};
use glfw::{Action, Context, Key};
use learn_opengl_with_rust::camera::{Camera, CameraMovement};
use learn_opengl_with_rust::shader_m::Shader_M;
use learn_opengl_with_rust::{size_of_floats, SIZE_OF_FLOAT};
use std::mem;

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

    let (mut window, events) = glfw
        .create_window(SCR_WIDTH as u32, SCR_HEIGHT as u32, "LearnOpenGL", glfw::WindowMode::Windowed)
        .expect("Failed to create GLFW window.");

    // Turn on all GLFW polling so that we can receive all WindowEvents
    window.set_all_polling(true);
    window.make_current();

    // Initialize glad: load all OpenGL function pointers
    // --------------------------------------------------
    gl::load(|e| glfw.get_proc_address_raw(e) as *const std::os::raw::c_void);

    let camera = Camera::camera_vec3(vec3(0.0, 0.5, 4.0));

    // Initialize the world state
    let mut state = State {
        camera,
        deltaTime: 0.0,
        lastFrame: 0.0,
        firstMouse: true,
        lastX: SCR_WIDTH / 2.0,
        lastY: SCR_HEIGHT / 2.0,
    };

    // Vertex Array Object id
    let mut instanceVBO: GLuint = 0;
    let mut quadVAO: GLuint = 0;
    let mut quadVBO: GLuint = 0;

    // build and compile our shader program
    // ------------------------------------
    // Shader program
    let shader = Shader_M::new(
        "examples/4-advanced_opengl/10_1-instancing_quads/10_1-instancing.vert",
        "examples/4-advanced_opengl/10_1-instancing_quads/10_1-instancing.frag",
    )
    .unwrap();

    // set up vertex data (and buffer(s)) and configure vertex attributes
    // ------------------------------------------------------------------
    #[rustfmt::skip]
        let quadVertices: [f32; 30] = [
        // positions     // colors
        -0.05,  0.05,  1.0, 0.0, 0.0,
         0.05, -0.05,  0.0, 1.0, 0.0,
        -0.05, -0.05,  0.0, 0.0, 1.0,

        -0.05,  0.05,  1.0, 0.0, 0.0,
         0.05, -0.05,  0.0, 1.0, 0.0,
         0.05,  0.05,  0.0, 1.0, 1.0
    ];

    let mut translations: Vec<Vec2> = vec![];
    let offset: f32 = 0.1;

    for y in (-10..10).step_by(2) {
        for x in (-10..10).step_by(2) {
            let translation = Vec2 {
                x: (x as f32 / 10.0f32) + offset,
                y: (y as f32 / 10.0f32) + offset,
            };
            translations.push(translation);
        }
    }

    unsafe {
        // configure global opengl state
        // -----------------------------
        gl::Enable(gl::DEPTH_TEST);

        // store instance data in an array buffer
        // --------------------------------------
        gl::GenBuffers(1, &mut instanceVBO);
        gl::BindBuffer(gl::ARRAY_BUFFER, instanceVBO);
        gl::BufferData(
            gl::ARRAY_BUFFER,
            (mem::size_of::<Vec2>() * 100) as GLsizeiptr,
            translations.as_ptr() as *const GLvoid,
            gl::STATIC_DRAW,
        );
        gl::BindBuffer(gl::ARRAY_BUFFER, 0);

        // quad VAO
        gl::GenVertexArrays(1, &mut quadVAO);
        gl::GenBuffers(1, &mut quadVBO);
        gl::BindVertexArray(quadVAO);
        gl::BindBuffer(gl::ARRAY_BUFFER, quadVBO);
        gl::BufferData(
            gl::ARRAY_BUFFER,
            size_of_floats!(quadVertices.len()) as GLsizeiptr,
            quadVertices.as_ptr() as *const GLvoid,
            gl::STATIC_DRAW,
        );
        gl::EnableVertexAttribArray(0);
        gl::VertexAttribPointer(0, 2, gl::FLOAT, gl::FALSE, size_of_floats!(5) as GLsizei, 0 as *const GLvoid);
        gl::EnableVertexAttribArray(1);
        gl::VertexAttribPointer(
            1,
            3,
            gl::FLOAT,
            gl::FALSE,
            size_of_floats!(5) as GLsizei,
            (2 * SIZE_OF_FLOAT) as *const GLvoid,
        );
        // also set instance data
        gl::EnableVertexAttribArray(2);
        gl::BindBuffer(gl::ARRAY_BUFFER, instanceVBO);
        // this attribute comes from a different vertex buffer
        gl::VertexAttribPointer(2, 2, gl::FLOAT, gl::FALSE, size_of_floats!(2) as GLsizei, 0 as *const GLvoid);
        gl::BindBuffer(gl::ARRAY_BUFFER, 0);
        // tell OpenGL this is an instanced vertex attribute.
        gl::VertexAttribDivisor(2, 1);
    }

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

            // draw 100 instanced quads
            shader.use_shader();
            gl::BindVertexArray(quadVAO);
            gl::DrawArraysInstanced(gl::TRIANGLES, 0, 6, 100); // 100 triangles of 6 vertices each
            gl::BindVertexArray(0);
        }

        window.swap_buffers();
    }

    // optional: de-allocate all resources once they've outlived their purpose:
    // ------------------------------------------------------------------------
    unsafe {
        gl::DeleteBuffers(1, &instanceVBO);
        gl::DeleteVertexArrays(1, &quadVAO);
        gl::DeleteBuffers(1, &quadVBO);
        gl::DeleteShader(shader.programId);
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

