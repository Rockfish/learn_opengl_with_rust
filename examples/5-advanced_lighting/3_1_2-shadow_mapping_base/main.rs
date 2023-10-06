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
use glam::{vec3, Mat4};
use glfw::{Action, Context, Key};
use image::ColorType;
use learn_opengl_with_rust::camera::{Camera, CameraMovement};
use learn_opengl_with_rust::shader::Shader;
use learn_opengl_with_rust::SIZE_OF_FLOAT;
use std::ffi::c_uint;

const SCR_WIDTH: f32 = 800.0;
const SCR_HEIGHT: f32 = 800.0;

struct State {
    camera: Camera,
    deltaTime: f32,
    lastFrame: f32,
    firstMouse: bool,
    lastX: f32,
    lastY: f32,
    gammaEnabled: bool,
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
        gammaEnabled: false,
    };

    // build and compile our shader program
    // ------------------------------------
    let shader = Shader::new(
        "examples/5-advanced_lighting/3_1_2-shadow_mapping_base/3_1_2-shadow_mapping.vert",
        "examples/5-advanced_lighting/3_1_2-shadow_mapping_base/3_1_2-shadow_mapping.frag",
        None,
    )
    .unwrap();

    let simpleDepthShader = Shader::new(
        "examples/5-advanced_lighting/3_1_2-shadow_mapping_base/3_1_2-shadow_mapping_depth.vert",
        "examples/5-advanced_lighting/3_1_2-shadow_mapping_base/3_1_2-shadow_mapping_depth.frag",
        None,
    )
    .unwrap();

    let debugDepthQuad = Shader::new(
        "examples/5-advanced_lighting/3_1_2-shadow_mapping_base/3_1_2-debug_quad.vert",
        "examples/5-advanced_lighting/3_1_2-shadow_mapping_base/3_1_2-debug_quad_depth.frag",
        None,
    )
    .unwrap();

    // set up vertex data (and buffer(s)) and configure vertex attributes
    // ------------------------------------------------------------------
    #[rustfmt::skip]
    let planeVertices: [f32; 48] = [
        // positions         // normals      // texcoords
         25.0, -0.5,  25.0,  0.0, 1.0, 0.0,  25.0,  0.0,
        -25.0, -0.5,  25.0,  0.0, 1.0, 0.0,   0.0,  0.0,
        -25.0, -0.5, -25.0,  0.0, 1.0, 0.0,   0.0, 25.0,

         25.0, -0.5,  25.0,  0.0, 1.0, 0.0,  25.0,  0.0,
        -25.0, -0.5, -25.0,  0.0, 1.0, 0.0,   0.0, 25.0,
         25.0, -0.5, -25.0,  0.0, 1.0, 0.0,  25.0, 25.0,
    ];

    // Vertex Array Object id
    let mut planeVAO: GLuint = 0;
    let mut planeVBO: GLuint = 0;
    let mut cubeVAO: GLuint = 0;
    #[allow(unused_variables, unused_mut)]
    let mut quadVAO: GLuint = 0;
    let mut depthMapFBO: GLuint = 0;
    let mut depthMap: GLuint = 0;

    unsafe {
        // configure global opengl state
        // -----------------------------
        gl::Enable(gl::DEPTH_TEST);

        // plane VAO
        gl::GenVertexArrays(1, &mut planeVAO);
        gl::GenBuffers(1, &mut planeVBO);
        gl::BindVertexArray(planeVAO);
        gl::BindBuffer(gl::ARRAY_BUFFER, planeVAO);
        gl::BufferData(
            gl::ARRAY_BUFFER,
            (planeVertices.len() * SIZE_OF_FLOAT) as GLsizeiptr,
            planeVertices.as_ptr() as *const GLvoid,
            gl::STATIC_DRAW,
        );
        gl::EnableVertexAttribArray(0);
        gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, (8 * SIZE_OF_FLOAT) as GLsizei, 0 as *const GLvoid);
        gl::EnableVertexAttribArray(1);
        gl::VertexAttribPointer(
            1,
            3,
            gl::FLOAT,
            gl::FALSE,
            (8 * SIZE_OF_FLOAT) as GLsizei,
            (3 * SIZE_OF_FLOAT) as *const GLvoid,
        );
        gl::EnableVertexAttribArray(2);
        gl::VertexAttribPointer(
            2,
            2,
            gl::FLOAT,
            gl::FALSE,
            (8 * SIZE_OF_FLOAT) as GLsizei,
            (6 * SIZE_OF_FLOAT) as *const GLvoid,
        );
        gl::BindVertexArray(0);
    }

    // load textures
    let woodTexture = loadTexture("resources/textures/wood.png", false);

    // configure depth map FBO
    // -----------------------
    let SHADOW_WIDTH = 1024;
    let SHADOW_HEIGHT = 1024;
    unsafe {
        gl::GenFramebuffers(1, &mut depthMapFBO);
        // create depth texture
        gl::GenTextures(1, &mut depthMap);
        gl::BindTexture(gl::TEXTURE_2D, depthMap);
        gl::TexImage2D(
            gl::TEXTURE_2D,
            0,
            gl::DEPTH_COMPONENT as GLint,
            SHADOW_WIDTH,
            SHADOW_HEIGHT,
            0,
            gl::DEPTH_COMPONENT,
            gl::FLOAT,
            0 as *const GLvoid,
        );
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::NEAREST as GLint);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::NEAREST as GLint);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT as GLint);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT as GLint);
        // attach depth texture as FBO's depth buffer
        gl::BindFramebuffer(gl::FRAMEBUFFER, depthMapFBO);
        gl::FramebufferTexture2D(gl::FRAMEBUFFER, gl::DEPTH_ATTACHMENT, gl::TEXTURE_2D, depthMap, 0);
        gl::DrawBuffer(gl::NONE);
        gl::ReadBuffer(gl::NONE);
        gl::BindFramebuffer(gl::FRAMEBUFFER, 0);
    }

    // shader configuration
    // --------------------
    shader.use_shader();
    shader.setInt("diffuseTexture", 0);
    shader.setInt("shadowMap", 1);
    debugDepthQuad.use_shader();
    debugDepthQuad.setInt("depthMap", 0);

    // lighting info
    // -------------
    let lightPos = vec3(-2.0, 4.0, -1.0);

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
            // ------
            gl::ClearColor(0.1, 0.1, 0.1, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);

            // 1. render depth of scene to texture (from light's perspective)
            // --------------------------------------------------------------
            let near_plane: f32 = 1.0;
            let far_plane: f32 = 7.5;

            let lightProjection = Mat4::orthographic_rh_gl(-10.0, 10.0, -10.0, 10.0, near_plane, far_plane);
            let lightView = Mat4::look_at_rh(lightPos, vec3(0.0, 0.0, 0.0), vec3(0.0, 1.0, 0.0));
            let lightSpaceMatrix = lightProjection * lightView;

            // render scene from light's point of view
            simpleDepthShader.use_shader();
            simpleDepthShader.setMat4("lightSpaceMatrix", &lightSpaceMatrix);

            gl::Viewport(0, 0, SHADOW_WIDTH, SHADOW_HEIGHT);
            gl::BindFramebuffer(gl::FRAMEBUFFER, depthMapFBO);
            gl::Clear(gl::DEPTH_BUFFER_BIT);
            gl::ActiveTexture(gl::TEXTURE0);
            gl::BindTexture(gl::TEXTURE_2D, woodTexture);
            renderScene(&simpleDepthShader, planeVAO, &mut cubeVAO);
            gl::BindFramebuffer(gl::FRAMEBUFFER, 0);

            // reset viewport
            gl::Viewport(0, 0, SCR_WIDTH as GLsizei, SCR_HEIGHT as GLsizei);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);

            // 2. render scene as normal using the generated depth/shadow map
            // --------------------------------------------------------------
            shader.use_shader();
            let projection = Mat4::perspective_rh_gl(state.camera.Zoom.to_radians(), SCR_WIDTH / SCR_HEIGHT, 0.1, 100.0);
            let view = state.camera.GetViewMatrix();
            shader.setMat4("projection", &projection);
            shader.setMat4("view", &view);
            // set light uniforms
            shader.setVec3("viewPos", &state.camera.Position);
            shader.setVec3("lightPos", &lightPos);
            shader.setMat4("lightSpaceMatrix", &lightSpaceMatrix);
            gl::ActiveTexture(gl::TEXTURE0);
            gl::BindTexture(gl::TEXTURE_2D, woodTexture);
            gl::ActiveTexture(gl::TEXTURE1);
            gl::BindTexture(gl::TEXTURE_2D, depthMap);
            renderScene(&shader, planeVAO, &mut cubeVAO);

            // render Depth map to quad for visual debugging
            // ---------------------------------------------
            debugDepthQuad.use_shader();
            debugDepthQuad.setFloat("near_plane", near_plane);
            debugDepthQuad.setFloat("far_plane", far_plane);
            gl::ActiveTexture(gl::TEXTURE0);
            gl::BindTexture(gl::TEXTURE_2D, depthMap);
            // renderQuad(&mut quadVAO);
        }

        window.swap_buffers();
    }

    // optional: de-allocate all resources once they've outlived their purpose:
    // ------------------------------------------------------------------------
    unsafe {
        gl::DeleteVertexArrays(1, &planeVAO);
        gl::DeleteBuffers(1, &planeVBO);
        gl::DeleteShader(simpleDepthShader.id);
        gl::DeleteShader(debugDepthQuad.id);
    }
}

// renders the 3D scene
// --------------------
fn renderScene(shader: &Shader, planeVAO: GLuint, cubeVAO: &mut GLuint) {
    // floor
    let model = Mat4::IDENTITY;
    shader.setMat4("model", &model);
    unsafe {
        gl::BindVertexArray(planeVAO);
        gl::DrawArrays(gl::TRIANGLES, 0, 6);
    }

    // cubes
    let mut model = Mat4::from_translation(vec3(0.0, 1.5, 0.0));
    model *= Mat4::from_scale(vec3(0.5, 0.5, 0.5));
    shader.setMat4("model", &model);
    renderCube(cubeVAO);

    let mut model = Mat4::from_translation(vec3(2.0, 0.0, 1.0));
    model *= Mat4::from_scale(vec3(0.5, 0.5, 0.5));
    shader.setMat4("model", &model);
    renderCube(cubeVAO);

    let mut model = Mat4::from_translation(vec3(-1.0, 0.0, 2.0));
    model *= Mat4::from_axis_angle(vec3(1.0, 0.0, 1.0).normalize(), 60.0f32.to_radians());
    model *= Mat4::from_scale(vec3(0.25, 0.25, 0.25));
    shader.setMat4("model", &model);
    renderCube(cubeVAO);
}

fn renderCube(cubeVAO: &mut GLuint) {
    // initialize (if necessary)
    if *cubeVAO == 0 {
        #[rustfmt::skip]
        let vertices: [f32; 288] = [
            // back face
            -1.0, -1.0, -1.0,  0.0,  0.0, -1.0, 0.0, 0.0, // bottom-left
             1.0,  1.0, -1.0,  0.0,  0.0, -1.0, 1.0, 1.0, // top-right
             1.0, -1.0, -1.0,  0.0,  0.0, -1.0, 1.0, 0.0, // bottom-right         
             1.0,  1.0, -1.0,  0.0,  0.0, -1.0, 1.0, 1.0, // top-right
            -1.0, -1.0, -1.0,  0.0,  0.0, -1.0, 0.0, 0.0, // bottom-left
            -1.0,  1.0, -1.0,  0.0,  0.0, -1.0, 0.0, 1.0, // top-left
            // front face
            -1.0, -1.0,  1.0,  0.0,  0.0,  1.0, 0.0, 0.0, // bottom-left
             1.0, -1.0,  1.0,  0.0,  0.0,  1.0, 1.0, 0.0, // bottom-right
             1.0,  1.0,  1.0,  0.0,  0.0,  1.0, 1.0, 1.0, // top-right
             1.0,  1.0,  1.0,  0.0,  0.0,  1.0, 1.0, 1.0, // top-right
            -1.0,  1.0,  1.0,  0.0,  0.0,  1.0, 0.0, 1.0, // top-left
            -1.0, -1.0,  1.0,  0.0,  0.0,  1.0, 0.0, 0.0, // bottom-left
            // left face
            -1.0,  1.0,  1.0, -1.0,  0.0,  0.0, 1.0, 0.0, // top-right
            -1.0,  1.0, -1.0, -1.0,  0.0,  0.0, 1.0, 1.0, // top-left
            -1.0, -1.0, -1.0, -1.0,  0.0,  0.0, 0.0, 1.0, // bottom-left
            -1.0, -1.0, -1.0, -1.0,  0.0,  0.0, 0.0, 1.0, // bottom-left
            -1.0, -1.0,  1.0, -1.0,  0.0,  0.0, 0.0, 0.0, // bottom-right
            -1.0,  1.0,  1.0, -1.0,  0.0,  0.0, 1.0, 0.0, // top-right
            // right face
             1.0,  1.0,  1.0,  1.0,  0.0,  0.0, 1.0, 0.0, // top-left
             1.0, -1.0, -1.0,  1.0,  0.0,  0.0, 0.0, 1.0, // bottom-right
             1.0,  1.0, -1.0,  1.0,  0.0,  0.0, 1.0, 1.0, // top-right         
             1.0, -1.0, -1.0,  1.0,  0.0,  0.0, 0.0, 1.0, // bottom-right
             1.0,  1.0,  1.0,  1.0,  0.0,  0.0, 1.0, 0.0, // top-left
             1.0, -1.0,  1.0,  1.0,  0.0,  0.0, 0.0, 0.0, // bottom-left     
            // bottom face
            -1.0, -1.0, -1.0,  0.0, -1.0,  0.0, 0.0, 1.0, // top-right
             1.0, -1.0, -1.0,  0.0, -1.0,  0.0, 1.0, 1.0, // top-left
             1.0, -1.0,  1.0,  0.0, -1.0,  0.0, 1.0, 0.0, // bottom-left
             1.0, -1.0,  1.0,  0.0, -1.0,  0.0, 1.0, 0.0, // bottom-left
            -1.0, -1.0,  1.0,  0.0, -1.0,  0.0, 0.0, 0.0, // bottom-right
            -1.0, -1.0, -1.0,  0.0, -1.0,  0.0, 0.0, 1.0, // top-right
            // top face
            -1.0,  1.0, -1.0,  0.0,  1.0,  0.0, 0.0, 1.0, // top-left
             1.0,  1.0,  1.0,  0.0,  1.0,  0.0, 1.0, 0.0, // bottom-right
             1.0,  1.0, -1.0,  0.0,  1.0,  0.0, 1.0, 1.0, // top-right     
             1.0,  1.0,  1.0,  0.0,  1.0,  0.0, 1.0, 0.0, // bottom-right
            -1.0,  1.0, -1.0,  0.0,  1.0,  0.0, 0.0, 1.0, // top-left
            -1.0,  1.0,  1.0,  0.0,  1.0,  0.0, 0.0, 0.0,  // bottom-left   
        ];

        unsafe {
            let mut cubeVBO: GLuint = 0;
            gl::GenVertexArrays(1, cubeVAO);
            gl::GenBuffers(1, &mut cubeVBO);
            // fill buffer
            gl::BindBuffer(gl::ARRAY_BUFFER, cubeVBO);
            gl::BufferData(
                gl::ARRAY_BUFFER,
                (vertices.len() * SIZE_OF_FLOAT) as GLsizeiptr,
                vertices.as_ptr() as *const GLvoid,
                gl::STATIC_DRAW,
            );

            gl::BindVertexArray(*cubeVAO);
            gl::EnableVertexAttribArray(0);
            gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, (8 * SIZE_OF_FLOAT) as GLsizei, 0 as *const GLvoid);
            gl::EnableVertexAttribArray(1);
            gl::VertexAttribPointer(
                1,
                3,
                gl::FLOAT,
                gl::FALSE,
                (8 * SIZE_OF_FLOAT) as GLsizei,
                (3 * SIZE_OF_FLOAT) as *const GLvoid,
            );
            gl::EnableVertexAttribArray(2);
            gl::VertexAttribPointer(
                2,
                2,
                gl::FLOAT,
                gl::FALSE,
                (8 * SIZE_OF_FLOAT) as GLsizei,
                (6 * SIZE_OF_FLOAT) as *const GLvoid,
            );
            gl::BindBuffer(gl::ARRAY_BUFFER, 0);
            gl::BindVertexArray(0);
        }
    }
    // render Cube
    unsafe {
        gl::BindVertexArray(*cubeVAO);
        gl::DrawArrays(gl::TRIANGLES, 0, 36);
        gl::BindVertexArray(0);
    }
}

fn renderQuad(quadVAO: &mut GLuint) {
    // initialize (if necessary)
    if *quadVAO == 0 {
        #[rustfmt::skip]
        let quadVertices: [f32; 20] = [
            // positions     // texture Coords
            -1.0,  1.0, 0.0, 0.0, 1.0,
            -1.0, -1.0, 0.0, 0.0, 0.0,
             1.0,  1.0, 0.0, 1.0, 1.0,
             1.0, -1.0, 0.0, 1.0, 0.0,
        ];

        // setup plane VAO
        unsafe {
            let mut quadVBO: GLuint = 0;
            gl::GenVertexArrays(1, quadVAO);
            gl::GenBuffers(1, &mut quadVBO);
            gl::BindVertexArray(*quadVAO);
            gl::BindBuffer(gl::ARRAY_BUFFER, quadVBO);
            gl::BufferData(
                gl::ARRAY_BUFFER,
                (quadVertices.len() * SIZE_OF_FLOAT) as GLsizeiptr,
                quadVertices.as_ptr() as *const GLvoid,
                gl::STATIC_DRAW,
            );
            gl::EnableVertexAttribArray(0);
            gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, (5 * SIZE_OF_FLOAT) as GLsizei, 0 as *const GLvoid);
            gl::EnableVertexAttribArray(1);
            gl::VertexAttribPointer(
                1,
                2,
                gl::FLOAT,
                gl::FALSE,
                (5 * SIZE_OF_FLOAT) as GLsizei,
                (3 * SIZE_OF_FLOAT) as *const GLvoid,
            );
        }
    }
    unsafe {
        gl::BindVertexArray(*quadVAO);
        gl::DrawArrays(gl::TRIANGLE_STRIP, 0, 4);
        gl::BindVertexArray(0);
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
        glfw::WindowEvent::Key(Key::Space, _, action, _) => {
            if action == Action::Press {
                state.gammaEnabled = !state.gammaEnabled
            }
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
fn loadTexture(path: &str, gammaCorrection: bool) -> GLuint {
    let mut texture_id: GLuint = 0;

    let img = image::open(path).expect("Texture failed to load");
    let (width, height) = (img.width() as GLsizei, img.height() as GLsizei);

    let color_type = img.color();
    // let data = img.into_rgb8().into_raw();

    unsafe {
        let mut internalFormat: c_uint = 0;
        let mut dataFormat: c_uint = 0;
        match color_type {
            ColorType::L8 => {
                internalFormat = gl::RED;
                dataFormat = gl::RED;
            }
            // ColorType::La8 => {}
            ColorType::Rgb8 => {
                internalFormat = if gammaCorrection { gl::SRGB } else { gl::RGB };
                dataFormat = gl::RGB;
            }
            ColorType::Rgba8 => {
                internalFormat = if gammaCorrection { gl::SRGB_ALPHA } else { gl::RGBA };
                dataFormat = gl::RGBA;
            }
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
            internalFormat as GLint,
            width,
            height,
            0,
            dataFormat,
            gl::UNSIGNED_BYTE,
            data.as_ptr() as *const GLvoid,
        );
        gl::GenerateMipmap(gl::TEXTURE_2D);

        // for this tutorial: use GL_CLAMP_TO_EDGE to prevent semi-transparent borders. Due to interpolation it takes texels from next repeat
        let param = if dataFormat == gl::RGBA { gl::CLAMP_TO_EDGE } else { gl::REPEAT };
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, param as GLint);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, param as GLint);

        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR_MIPMAP_LINEAR as GLint);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as GLint);
    }

    texture_id
}
