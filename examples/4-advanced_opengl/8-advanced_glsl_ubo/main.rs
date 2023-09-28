#![feature(is_sorted)]
#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(unused_assignments)]
#![allow(clippy::zero_ptr)]
#![allow(clippy::assign_op_pattern)]

extern crate glfw;

use glad_gl::gl;
use glad_gl::gl::{GLint, GLintptr, GLsizei, GLsizeiptr, GLuint, GLvoid};
use glam::{vec3, Mat4};
use glfw::{Action, Context, Key};
use image::ColorType;
use learn_opengl_with_rust::camera::{Camera, CameraMovement};
use learn_opengl_with_rust::shader_m::Shader_M;
use learn_opengl_with_rust::{c_string, size_of_floats, SIZE_OF_FLOAT};
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
    let shaderRed = Shader_M::new(
        "examples/4-advanced_opengl/8-advanced_glsl_ubo/8-advanced_glsl.vert",
        "examples/4-advanced_opengl/8-advanced_glsl_ubo/8-red.frag",
    )
    .unwrap();
    let shaderGreen = Shader_M::new(
        "examples/4-advanced_opengl/8-advanced_glsl_ubo/8-advanced_glsl.vert",
        "examples/4-advanced_opengl/8-advanced_glsl_ubo/8-green.frag",
    )
    .unwrap();
    let shaderBlue = Shader_M::new(
        "examples/4-advanced_opengl/8-advanced_glsl_ubo/8-advanced_glsl.vert",
        "examples/4-advanced_opengl/8-advanced_glsl_ubo/8-blue.frag",
    )
    .unwrap();
    let shaderYellow = Shader_M::new(
        "examples/4-advanced_opengl/8-advanced_glsl_ubo/8-advanced_glsl.vert",
        "examples/4-advanced_opengl/8-advanced_glsl_ubo/8-yellow.frag",
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
        -0.5,  0.5, -0.5,
    ];

    // Vertex Array Object id
    let mut cubeVAO: GLuint = 0;
    let mut cubeVBO: GLuint = 0;
    let mut uboMatrices: GLuint = 0;

    let size_of_mat4 = mem::size_of::<[f32; 16]>();

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
        gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, (3 * SIZE_OF_FLOAT) as GLsizei, 0 as *const GLvoid);

        // configure a uniform buffer object
        // ---------------------------------
        // first. We get the relevant block indices
        let c_string = c_string!("Matrices");
        let uniformBlockIndexRed = gl::GetUniformBlockIndex(shaderRed.id, c_string.as_ptr());
        let uniformBlockIndexGreen = gl::GetUniformBlockIndex(shaderGreen.id, c_string.as_ptr());
        let uniformBlockIndexBlue = gl::GetUniformBlockIndex(shaderBlue.id, c_string.as_ptr());
        let uniformBlockIndexYellow = gl::GetUniformBlockIndex(shaderYellow.id, c_string.as_ptr());

        // then we link each shader's uniform block to this uniform binding point
        gl::UniformBlockBinding(shaderRed.id, uniformBlockIndexRed, 0);
        gl::UniformBlockBinding(shaderGreen.id, uniformBlockIndexGreen, 0);
        gl::UniformBlockBinding(shaderBlue.id, uniformBlockIndexBlue, 0);
        gl::UniformBlockBinding(shaderYellow.id, uniformBlockIndexYellow, 0);

        // Now actually create the buffer
        gl::GenBuffers(1, &mut uboMatrices);
        gl::BindBuffer(gl::UNIFORM_BUFFER, uboMatrices);
        gl::BufferData(gl::UNIFORM_BUFFER, (2 * size_of_mat4) as GLsizeiptr, std::ptr::null(), gl::STATIC_DRAW);
        gl::BindBuffer(gl::UNIFORM_BUFFER, 0);

        // define the range of the buffer that links to a uniform binding point
        gl::BindBufferRange(gl::UNIFORM_BUFFER, 0, uboMatrices, 0, (2 * size_of_mat4) as GLsizeiptr);

        // store the projection matrix (we only do this once now) (note: we're not using zoom anymore by changing the FoV)
        let projection = Mat4::perspective_rh_gl(45.0, SCR_WIDTH / SCR_HEIGHT, 0.1, 100.0);
        gl::BindBuffer(gl::UNIFORM_BUFFER, uboMatrices);
        gl::BufferSubData(
            gl::UNIFORM_BUFFER,
            0,
            size_of_mat4 as GLsizeiptr,
            projection.to_cols_array().as_ptr() as *const GLvoid,
        );
        gl::BindBuffer(gl::UNIFORM_BUFFER, 0);
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

            // set the view matrix in the sub data block - we only have to do this once per loop iteration.
            let view = state.camera.GetViewMatrix();
            gl::BindBuffer(gl::UNIFORM_BUFFER, uboMatrices);
            gl::BufferSubData(
                gl::UNIFORM_BUFFER,
                size_of_mat4 as GLintptr,
                size_of_mat4 as GLsizeiptr,
                view.to_cols_array().as_ptr() as *const GLvoid,
            );
            gl::BindBuffer(gl::UNIFORM_BUFFER, 0);

            // draw 4 cubes
            // RED
            gl::BindVertexArray(cubeVAO);
            shaderRed.use_shader();
            let model = Mat4::from_translation(vec3(-0.75, 0.75, 0.0)); // move top-left
            shaderRed.setMat4("model", &model);
            gl::DrawArrays(gl::TRIANGLES, 0, 36);

            // GREEN
            gl::BindVertexArray(cubeVAO);
            shaderGreen.use_shader();
            let model = Mat4::from_translation(vec3(0.75, 0.75, 0.0)); // move top-left
            shaderGreen.setMat4("model", &model);
            gl::DrawArrays(gl::TRIANGLES, 0, 36);

            // YELLOW
            gl::BindVertexArray(cubeVAO);
            shaderYellow.use_shader();
            let model = Mat4::from_translation(vec3(-0.75, -0.75, 0.0)); // move top-left
            shaderYellow.setMat4("model", &model);
            gl::DrawArrays(gl::TRIANGLES, 0, 36);

            // BLUE
            gl::BindVertexArray(cubeVAO);
            shaderBlue.use_shader();
            let model = Mat4::from_translation(vec3(0.75, -0.75, 0.0)); // move top-left
            shaderBlue.setMat4("model", &model);
            gl::DrawArrays(gl::TRIANGLES, 0, 36);
        }

        window.swap_buffers();
    }

    // optional: de-allocate all resources once they've outlived their purpose:
    // ------------------------------------------------------------------------
    unsafe {
        gl::DeleteVertexArrays(1, &cubeVAO);
        gl::DeleteBuffers(1, &cubeVBO);
        gl::DeleteShader(shaderRed.id);
        gl::DeleteShader(shaderGreen.id);
        gl::DeleteShader(shaderYellow.id);
        gl::DeleteShader(shaderBlue.id);
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

            gl::TexImage2D(
                gl::TEXTURE_CUBE_MAP_POSITIVE_X + i as u32,
                0,
                gl::RGB as GLint,
                width,
                height,
                0,
                gl::RGB,
                gl::UNSIGNED_BYTE,
                data.as_ptr() as *const GLvoid,
            );
        }

        gl::TexParameteri(gl::TEXTURE_CUBE_MAP, gl::TEXTURE_MIN_FILTER, gl::LINEAR as GLint);
        gl::TexParameteri(gl::TEXTURE_CUBE_MAP, gl::TEXTURE_MAG_FILTER, gl::LINEAR as GLint);
        gl::TexParameteri(gl::TEXTURE_CUBE_MAP, gl::TEXTURE_WRAP_S, gl::CLAMP_TO_EDGE as GLint);
        gl::TexParameteri(gl::TEXTURE_CUBE_MAP, gl::TEXTURE_WRAP_T, gl::CLAMP_TO_EDGE as GLint);
        gl::TexParameteri(gl::TEXTURE_CUBE_MAP, gl::TEXTURE_WRAP_R, gl::CLAMP_TO_EDGE as GLint);
    }
    textureID
}
