#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(unused_assignments)]
#![allow(clippy::zero_ptr)]
#![allow(clippy::assign_op_pattern)]

extern crate glfw;

use glad_gl::gl;
use glad_gl::gl::{GLint, GLsizei, GLuint, GLvoid};
use glam::*;
use glfw::{Action, Context, Key};
use image::ColorType;
use learn_opengl_with_rust::camera::{Camera, CameraMovement};
use learn_opengl_with_rust::model::Model;
use learn_opengl_with_rust::shader_m::Shader_M;

const SCR_WIDTH: f32 = 800.0;
const SCR_HEIGHT: f32 = 800.0;

// Struct for passing state between the window loop and the event handler.
struct State {
    camera: Camera,
    lightPos: Vec3,
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
        lightPos: vec3(1.2, 1.0, 2.0),
        deltaTime: 0.0,
        lastFrame: 0.0,
        firstMouse: true,
        lastX: SCR_WIDTH / 2.0,
        lastY: SCR_HEIGHT / 2.0,
    };

    // configure global opengl state
    // -----------------------------
    unsafe {
        gl::Enable(gl::DEPTH_TEST);
    }

    // build and compile our shaders
    let mut ourShader = Shader_M::new();
    ourShader
        .build(
            "examples/3-model_loading/1-model_loading/1-model_loading.vert",
            "examples/3-model_loading/1-model_loading/1-model_loading.frag",
        )
        .unwrap();

    let ourModel = Model::new("resources/objects/cyborg/cyborg.obj", false, false);
    // let ourModel = Model::new("resources/objects/backpack/backpack.obj", false, true);
    // let ourModel = Model::new("/Users/john/Dev_Rust/Repos/russimp/models/OBJ/cube.obj", false, false);
    // let ourModel = Model::new("/Users/john/Dev_Rust/Dev/Models/Oyanirami0.3ds", false, false);

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
            gl::ClearColor(0.05, 0.05, 0.05, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);

            // be sure to activate shader when setting uniforms/drawing objects
            ourShader.use_shader();

            // view/projection transformations
            let projection = Mat4::perspective_rh_gl(state.camera.Zoom.to_radians(), SCR_WIDTH / SCR_HEIGHT, 0.1, 100.0);
            let view = state.camera.GetViewMatrix();
            ourShader.setMat4("projection", &projection);
            ourShader.setMat4("view", &view);

            let mut model = Mat4::from_translation(vec3(0.0, 0.0, 0.0));
            model = model * Mat4::from_scale(vec3(1.0, 1.0, 1.0));
            ourShader.setMat4("model", &model);

            ourModel.Draw(&ourShader);
        }

        window.swap_buffers();
    }

    // optional: de-allocate all resources once they've outlived their purpose:
    // ------------------------------------------------------------------------
    unsafe {
        gl::DeleteProgram(ourShader.programId);
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

// utility function for loading a 2D texture from file
// ---------------------------------------------------
fn loadTexture(path: &str) -> GLuint {
    let mut texture_id: GLuint = 0;

    let img = image::open(path).expect("Texture failed to load");
    let (width, height) = (img.width() as GLsizei, img.height() as GLsizei);

    let color_type = img.color();
    // let data = img.into_rgb8().into_raw();

    unsafe {
        let format = match color_type {
            ColorType::L8 => gl::RED,
            // ColorType::La8 => {}
            ColorType::Rgb8 => gl::RGB,
            ColorType::Rgba8 => gl::RGBA,
            // ColorType::L16 => {}
            // ColorType::La16 => {}
            // ColorType::Rgb16 => {}
            // ColorType::Rgba16 => {}
            // ColorType::Rgb32F => {}
            // ColorType::Rgba32F => {}
            _ => panic!("no mapping for color type"),
        };

        let data = match color_type {
            ColorType::L8 => img.into_rgb8().into_raw(),
            // ColorType::La8 => {}
            ColorType::Rgb8 => img.into_rgb8().into_raw(),
            ColorType::Rgba8 => img.into_rgba8().into_raw(),
            // ColorType::L16 => {}
            // ColorType::La16 => {}
            // ColorType::Rgb16 => {}
            // ColorType::Rgba16 => {}
            // ColorType::Rgb32F => {}
            // ColorType::Rgba32F => {}
            _ => panic!("no mapping for color type"),
        };

        gl::GenTextures(1, &mut texture_id);
        gl::BindTexture(gl::TEXTURE_2D, texture_id);

        gl::TexImage2D(
            gl::TEXTURE_2D,
            0,
            format as GLint,
            width,
            height,
            0,
            format,
            gl::UNSIGNED_BYTE,
            data.as_ptr() as *const GLvoid,
        );
        gl::GenerateMipmap(gl::TEXTURE_2D);

        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT as GLint);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT as GLint);

        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR_MIPMAP_LINEAR as GLint);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as GLint);
    }

    texture_id
}
