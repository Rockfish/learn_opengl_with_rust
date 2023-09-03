#![feature(is_sorted)]
#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(unused_assignments)]
#![allow(clippy::zero_ptr)]
#![allow(clippy::assign_op_pattern)]

extern crate glfw;

use glad_gl::gl;
use glad_gl::gl::{GLint, GLsizei, GLsizeiptr, GLuint, GLvoid};
use glam::{vec3, Mat4, Mat3};
use glfw::{Action, Context, Key};
use image::ColorType;
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

    // Vertex Array Object id
    let mut cubeVAO: GLuint = 0;
    let mut cubeVBO: GLuint = 0;
    let mut skyboxVAO: GLuint = 0;
    let mut skyboxVBO: GLuint = 0;

    // Texture ids
    let mut cubeTexture: GLuint = 0;

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

    // build and compile our shader program
    // ------------------------------------
    let shader = Shader_M::new(
        "examples/4-advanced_opengl/6_1-cubemaps_skybox/6_1-cubemaps.vert",
        "examples/4-advanced_opengl/6_1-cubemaps_skybox/6_1-cubemaps.frag",
    )
    .unwrap();
    let skyboxShader = Shader_M::new(
        "examples/4-advanced_opengl/6_1-cubemaps_skybox/6_1-skybox.vert",
        "examples/4-advanced_opengl/6_1-cubemaps_skybox/6_1-skybox.frag",
    )
    .unwrap();

    // set up vertex data (and buffer(s)) and configure vertex attributes
    // ------------------------------------------------------------------
    #[rustfmt::skip]
    let cubeVertices: [f32; 180] = [
        // positions       // texture Coords
        -0.5, -0.5, -0.5,  0.0, 0.0,
         0.5, -0.5, -0.5,  1.0, 0.0,
         0.5,  0.5, -0.5,  1.0, 1.0,
         0.5,  0.5, -0.5,  1.0, 1.0,
        -0.5,  0.5, -0.5,  0.0, 1.0,
        -0.5, -0.5, -0.5,  0.0, 0.0,

        -0.5, -0.5,  0.5,  0.0, 0.0,
         0.5, -0.5,  0.5,  1.0, 0.0,
         0.5,  0.5,  0.5,  1.0, 1.0,
         0.5,  0.5,  0.5,  1.0, 1.0,
        -0.5,  0.5,  0.5,  0.0, 1.0,
        -0.5, -0.5,  0.5,  0.0, 0.0,

        -0.5,  0.5,  0.5,  1.0, 0.0,
        -0.5,  0.5, -0.5,  1.0, 1.0,
        -0.5, -0.5, -0.5,  0.0, 1.0,
        -0.5, -0.5, -0.5,  0.0, 1.0,
        -0.5, -0.5,  0.5,  0.0, 0.0,
        -0.5,  0.5,  0.5,  1.0, 0.0,

         0.5,  0.5,  0.5,  1.0, 0.0,
         0.5,  0.5, -0.5,  1.0, 1.0,
         0.5, -0.5, -0.5,  0.0, 1.0,
         0.5, -0.5, -0.5,  0.0, 1.0,
         0.5, -0.5,  0.5,  0.0, 0.0,
         0.5,  0.5,  0.5,  1.0, 0.0,

        -0.5, -0.5, -0.5,  0.0, 1.0,
         0.5, -0.5, -0.5,  1.0, 1.0,
         0.5, -0.5,  0.5,  1.0, 0.0,
         0.5, -0.5,  0.5,  1.0, 0.0,
        -0.5, -0.5,  0.5,  0.0, 0.0,
        -0.5, -0.5, -0.5,  0.0, 1.0,

        -0.5,  0.5, -0.5,  0.0, 1.0,
         0.5,  0.5, -0.5,  1.0, 1.0,
         0.5,  0.5,  0.5,  1.0, 0.0,
         0.5,  0.5,  0.5,  1.0, 0.0,
        -0.5,  0.5,  0.5,  0.0, 0.0,
        -0.5,  0.5, -0.5,  0.0, 1.0
    ];

    #[rustfmt::skip]
    let skyboxVertices: [f32; 108] = [
        // positions          
        -1.0,  1.0, -1.0,
        -1.0, -1.0, -1.0,
         1.0, -1.0, -1.0,
         1.0, -1.0, -1.0,
         1.0,  1.0, -1.0,
        -1.0,  1.0, -1.0,

        -1.0, -1.0,  1.0,
        -1.0, -1.0, -1.0,
        -1.0,  1.0, -1.0,
        -1.0,  1.0, -1.0,
        -1.0,  1.0,  1.0,
        -1.0, -1.0,  1.0,

         1.0, -1.0, -1.0,
         1.0, -1.0,  1.0,
         1.0,  1.0,  1.0,
         1.0,  1.0,  1.0,
         1.0,  1.0, -1.0,
         1.0, -1.0, -1.0,

        -1.0, -1.0,  1.0,
        -1.0,  1.0,  1.0,
         1.0,  1.0,  1.0,
         1.0,  1.0,  1.0,
         1.0, -1.0,  1.0,
        -1.0, -1.0,  1.0,

        -1.0,  1.0, -1.0,
         1.0,  1.0, -1.0,
         1.0,  1.0,  1.0,
         1.0,  1.0,  1.0,
        -1.0,  1.0,  1.0,
        -1.0,  1.0, -1.0,

        -1.0, -1.0, -1.0,
        -1.0, -1.0,  1.0,
         1.0, -1.0, -1.0,
         1.0, -1.0, -1.0,
        -1.0, -1.0,  1.0,
         1.0, -1.0,  1.0,
    ];

    unsafe {
        // configure global opengl state
        // -----------------------------
        gl::Enable(gl::DEPTH_TEST);

        // cube VAO
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
        gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, size_of_floats!(5) as GLsizei, 0 as *const GLvoid);
        gl::EnableVertexAttribArray(1);
        gl::VertexAttribPointer(
            1,
            2,
            gl::FLOAT,
            gl::FALSE,
            size_of_floats!(5) as GLsizei,
            (3 * SIZE_OF_FLOAT) as *const GLvoid,
        );

        // skybox VAO
        gl::GenVertexArrays(1, &mut skyboxVAO);
        gl::GenBuffers(1, &mut skyboxVBO);
        gl::BindVertexArray(skyboxVAO);
        gl::BindBuffer(gl::ARRAY_BUFFER, skyboxVAO);
        gl::BufferData(
            gl::ARRAY_BUFFER,
            size_of_floats!(skyboxVertices.len()) as GLsizeiptr,
            skyboxVertices.as_ptr() as *const GLvoid,
            gl::STATIC_DRAW,
        );
        gl::EnableVertexAttribArray(0);
        gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, size_of_floats!(3) as GLsizei, 0 as *const GLvoid);
    }

    cubeTexture = loadTexture("resources/textures/container.jpg");

    let faces = vec!(
        "resources/textures/skybox/right.jpg",
        "resources/textures/skybox/left.jpg",
        "resources/textures/skybox/top.jpg",
        "resources/textures/skybox/bottom.jpg",
        "resources/textures/skybox/front.jpg",
        "resources/textures/skybox/back.jpg",
    );
    let cubemapTexture = loadCubemap(faces);

    // shader configuration
    shader.use_shader();
    shader.setInt("texture1", 0);

    skyboxShader.use_shader();
    skyboxShader.setInt("skybox", 0);

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

            // draw scene as normal
            shader.use_shader();
            let view = state.camera.GetViewMatrix();
            let projection = Mat4::perspective_rh_gl(state.camera.Zoom.to_radians(), SCR_WIDTH / SCR_HEIGHT, 0.1, 100.0);
            shader.setMat4("model", &Mat4::IDENTITY);
            shader.setMat4("view", &view);
            shader.setMat4("projection", &projection);

            // cubes
            gl::BindVertexArray(cubeVAO);
            gl::ActiveTexture(gl::TEXTURE0);
            gl::BindTexture(gl::TEXTURE_2D, cubeTexture);
            gl::DrawArrays(gl::TRIANGLES, 0, 36);
            gl::BindVertexArray(0);

            // draw skybox as last
            gl::DepthFunc(gl::LEQUAL);  // change depth function so depth test passes when values are equal to depth buffer's content
            skyboxShader.use_shader();
            // remove translation from the view matrix
            let view = state.camera.GetViewMatrix();
            let view= Mat3::from_mat4(view);
            let view = Mat4::from_mat3(view);
            skyboxShader.setMat4("view", &view);
            skyboxShader.setMat4("projection", &projection);

            // skybox cube
            gl::BindVertexArray(skyboxVAO);
            gl::ActiveTexture(gl::TEXTURE0);
            gl::BindTexture(gl::TEXTURE_CUBE_MAP, cubemapTexture);
            gl::DrawArrays(gl::TRIANGLES, 0, 36);
            gl::BindVertexArray(0);
            gl::DepthFunc(gl::LESS);
        }

        window.swap_buffers();
    }

    // optional: de-allocate all resources once they've outlived their purpose:
    // ------------------------------------------------------------------------
    unsafe {
        gl::DeleteVertexArrays(1, &cubeVAO);
        gl::DeleteVertexArrays(1, &skyboxVAO);
        gl::DeleteBuffers(1, &cubeVBO);
        gl::DeleteBuffers(1, &skyboxVBO);
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

// utility function for loading a 2D texture from file
// ---------------------------------------------------
fn loadTexture(path: &str) -> GLuint {
    let mut texture_id: GLuint = 0;

    let img = image::open(path).expect("Texture failed to load");
    let (width, height) = (img.width() as GLsizei, img.height() as GLsizei);
    let color_type = img.color();

    unsafe {
        let format = match color_type {
            ColorType::L8 => gl::RED,
            ColorType::Rgb8 => gl::RGB,
            ColorType::Rgba8 => gl::RGBA,
            _ => panic!("no mapping for color type"),
        };

        let data = match color_type {
            ColorType::L8 => img.into_rgb8().into_raw(),
            ColorType::Rgb8 => img.into_rgb8().into_raw(),
            ColorType::Rgba8 => img.into_rgba8().into_raw(),
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

        // for this tutorial: use gl::CLAMP_TO_EDGE to prevent semi-transparent borders. Due to interpolation it takes texels from next repeat
        let param = if format == gl::RGBA { gl::CLAMP_TO_EDGE } else { gl::REPEAT };
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, param as GLint);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, param as GLint);

        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR_MIPMAP_LINEAR as GLint);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as GLint);
    }

    texture_id
}

// loads a cubemap texture from 6 individual texture faces
// order:
// +X (right)
// -X (left)
// +Y (top)
// -Y (bottom)
// +Z (front)
// -Z (back)
// -------------------------------------------------------
pub fn loadCubemap(faces: Vec<&str>) -> u32 {
    let mut textureID: GLuint = 0;
    unsafe {
        gl::GenTextures(1, &mut textureID);
        gl::BindTexture(gl::TEXTURE_CUBE_MAP, textureID);

        for (i, path) in faces.iter().enumerate() {
            let img = image::open(path).expect("Texture failed to load");
            let (width, height) = (img.width() as GLsizei, img.height() as GLsizei);
            let color_type = img.color();

            // let format = match color_type {
            //     ColorType::L8 => gl::RED,
            //     ColorType::Rgb8 => gl::RGB,
            //     ColorType::Rgba8 => gl::RGBA,
            //     _ => panic!("no mapping for color type"),
            // };

            let data = match color_type {
                ColorType::L8 => img.into_rgb8().into_raw(),
                ColorType::Rgb8 => img.into_rgb8().into_raw(),
                ColorType::Rgba8 => img.into_rgba8().into_raw(),
                _ => panic!("no mapping for color type"),
            };

            gl::TexImage2D(gl::TEXTURE_CUBE_MAP_POSITIVE_X + i as u32, 0, gl::RGB as GLint, width, height, 0, gl::RGB, gl::UNSIGNED_BYTE, data.as_ptr() as *const GLvoid);
        }

        gl::TexParameteri(gl::TEXTURE_CUBE_MAP, gl::TEXTURE_MIN_FILTER, gl::LINEAR as GLint);
        gl::TexParameteri(gl::TEXTURE_CUBE_MAP, gl::TEXTURE_MAG_FILTER, gl::LINEAR as GLint);
        gl::TexParameteri(gl::TEXTURE_CUBE_MAP, gl::TEXTURE_WRAP_S, gl::CLAMP_TO_EDGE as GLint);
        gl::TexParameteri(gl::TEXTURE_CUBE_MAP, gl::TEXTURE_WRAP_T, gl::CLAMP_TO_EDGE as GLint);
        gl::TexParameteri(gl::TEXTURE_CUBE_MAP, gl::TEXTURE_WRAP_R, gl::CLAMP_TO_EDGE as GLint);
    }
    textureID
}
