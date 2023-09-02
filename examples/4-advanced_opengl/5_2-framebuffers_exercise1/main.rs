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
use learnopengl_lib::camera::{Camera, CameraMovement};
use learnopengl_lib::shader_m::Shader_M;
use learnopengl_lib::{size_of_floats, SIZE_OF_FLOAT};
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
    let mut planeVAO: GLuint = 0;
    let mut planeVBO: GLuint = 0;
    let mut quadVAO: GLuint = 0;
    let mut quadVBO: GLuint = 0;

    // Texture ids
    let mut cubeTexture: GLuint = 0;
    let mut floorTexture: GLuint = 0;

    // Shader program
    let mut shader = Shader_M::new();
    let mut screenShader = Shader_M::new();

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

    unsafe {
        // configure global opengl state
        // -----------------------------
        gl::Enable(gl::DEPTH_TEST);

        // build and compile our shader program
        // ------------------------------------
        shader
            .build(
                "examples/4-advanced_opengl/5_2-framebuffers_exercise1/5_2-framebuffers.vert",
                "examples/4-advanced_opengl/5_2-framebuffers_exercise1/5_2-framebuffers.frag",
            )
            .unwrap();

        screenShader
            .build(
                "examples/4-advanced_opengl/5_2-framebuffers_exercise1/5_2-framebuffers_screen.vert",
                "examples/4-advanced_opengl/5_2-framebuffers_exercise1/5_2-framebuffers_screen.frag",
                // "examples/4-advanced_opengl/5_2-framebuffers_exercise1/5_2-framebuffers_screen_greyscale.frag",
                // "examples/4-advanced_opengl/5_2-framebuffers_exercise1/5_2-framebuffers_screen_sharpening.frag",
                // "examples/4-advanced_opengl/5_2-framebuffers_exercise1/5_2-framebuffers_screen_blur.frag",
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
            let planeVertices: [f32; 30] = [
            // positions       // texture Coords
             5.0, -0.5,  5.0,  2.0, 0.0,
            -5.0, -0.5,  5.0,  0.0, 0.0,
            -5.0, -0.5, -5.0,  0.0, 2.0,

             5.0, -0.5,  5.0,  2.0, 0.0,
            -5.0, -0.5, -5.0,  0.0, 2.0,
             5.0, -0.5, -5.0,  2.0, 2.0
        ];

        #[rustfmt::skip]
        let quadVertices: [f32; 24] = [ // vertex attributes for a quad that fills the entire screen in Normalized Device Coordinates.
            // positions   // texCoords
            -1.0,  1.0,  0.0, 1.0,
            -1.0, -1.0,  0.0, 0.0,
             1.0, -1.0,  1.0, 0.0,

            -1.0,  1.0,  0.0, 1.0,
             1.0, -1.0,  1.0, 0.0,
             1.0,  1.0,  1.0, 1.0
        ];

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
        gl::BindVertexArray(0);

        // plane VAO
        gl::GenVertexArrays(1, &mut planeVAO);
        gl::GenBuffers(1, &mut planeVBO);
        gl::BindVertexArray(planeVAO);
        gl::BindBuffer(gl::ARRAY_BUFFER, planeVAO);
        gl::BufferData(
            gl::ARRAY_BUFFER,
            size_of_floats!(planeVertices.len()) as GLsizeiptr,
            planeVertices.as_ptr() as *const GLvoid,
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

        // screen quad VAO
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
        gl::BindVertexArray(0);
    }

    // load textures
    cubeTexture = loadTexture("resources/textures/container.jpg");
    floorTexture = loadTexture("resources/textures/metal.png");

    // shader configuration
    shader.use_shader();
    shader.setInt("texture1", 0);

    screenShader.use_shader();
    screenShader.setInt("screenTexture", 0);

    // framebuffer configuration
    // -------------------------
    let mut framebuffer: GLuint = 0;
    let mut textureColorbuffer: GLuint = 0;
    let mut rbo: GLuint = 0;

    unsafe {
        gl::GenFramebuffers(1, &mut framebuffer);
        gl::BindFramebuffer(gl::FRAMEBUFFER, framebuffer);
        // create a color attachment texture
        gl::GenTextures(1, &mut textureColorbuffer);
        gl::BindTexture(gl::TEXTURE_2D, textureColorbuffer);
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
        gl::FramebufferTexture2D(gl::FRAMEBUFFER, gl::COLOR_ATTACHMENT0, gl::TEXTURE_2D, textureColorbuffer, 0);
        // create a renderbuffer object for depth and stencil attachment (we won't be sampling these)
        gl::GenRenderbuffers(1, &mut rbo);
        gl::BindRenderbuffer(gl::RENDERBUFFER, rbo);
        // use a single renderbuffer object for both a depth AND stencil buffer.
        gl::RenderbufferStorage(gl::RENDERBUFFER, gl::DEPTH24_STENCIL8, SCR_WIDTH as GLsizei, SCR_HEIGHT as GLsizei);
        // now actually attach it
        gl::FramebufferRenderbuffer(gl::FRAMEBUFFER, gl::DEPTH_STENCIL_ATTACHMENT, gl::RENDERBUFFER, rbo);

        // now that we actually created the framebuffer and added all attachments we want to check if it is actually complete now
        if gl::CheckFramebufferStatus(gl::FRAMEBUFFER) != gl::FRAMEBUFFER_COMPLETE {
            println!("ERROR::FRAMEBUFFER:: Framebuffer is not complete!");
        }
        gl::BindFramebuffer(gl::FRAMEBUFFER, 0);
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
            // first render pass: mirror texture.
            // bind to framebuffer and draw to color texture as we normally
            // would, but with the view camera reversed.
            // bind to framebuffer and draw scene as we normally would to color texture
            // ------------------------------------------------------------------------
            gl::BindFramebuffer(gl::FRAMEBUFFER, framebuffer);
            gl::Enable(gl::DEPTH_TEST); // enable depth testing (is disabled for rendering screen-space quad)

            // make sure we clear the framebuffer's content
            gl::ClearColor(0.1, 0.1, 0.1, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);

            shader.use_shader();
            // rotate the camera's yaw 180 degrees around
            state.camera.Yaw += 180.0;
            // call this to make sure it updates its camera vectors, note that we disable pitch constrains for this specific case (otherwise we can't reverse camera's pitch values)
            state.camera.ProcessMouseMovement(0.0, 0.0, false);
            let view = state.camera.GetViewMatrix();
            // reset it back to its original orientation
            state.camera.Yaw -= 180.0;
            state.camera.ProcessMouseMovement(0.0, 0.0, true);
            let projection = Mat4::perspective_rh_gl(state.camera.Zoom.to_radians(), SCR_WIDTH / SCR_HEIGHT, 0.1, 100.0);
            shader.setMat4("projection", &projection);
            shader.setMat4("view", &view);

            // cubes
            gl::BindVertexArray(cubeVAO);
            gl::ActiveTexture(gl::TEXTURE0);
            gl::BindTexture(gl::TEXTURE_2D, cubeTexture);
            let model = Mat4::from_translation(vec3(-1.0, 0.0, -1.0));
            shader.setMat4("model", &model);
            gl::DrawArrays(gl::TRIANGLES, 0, 36);
            let model = Mat4::from_translation(vec3(2.0, 0.0, 0.0));
            shader.setMat4("model", &model);
            gl::DrawArrays(gl::TRIANGLES, 0, 36);

            // floor
            gl::BindVertexArray(planeVAO);
            gl::BindTexture(gl::TEXTURE_2D, floorTexture);
            // gl::ActiveTexture(gl::TEXTURE0);
            shader.setMat4("model", &Mat4::IDENTITY);
            gl::DrawArrays(gl::TRIANGLES, 0, 6);
            gl::BindVertexArray(0);

            // second render pass: draw as normal
            // ----------------------------------
            gl::BindFramebuffer(gl::FRAMEBUFFER, 0);
            gl::ClearColor(1.0, 1.0, 1.0, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);

            let view = state.camera.GetViewMatrix();
            shader.setMat4("view", &view);

            // cubes
            gl::BindVertexArray(cubeVAO);
            gl::ActiveTexture(gl::TEXTURE0);
            gl::BindTexture(gl::TEXTURE_2D, cubeTexture);
            let model = Mat4::from_translation(vec3(-1.0, 0.0, -1.0));
            shader.setMat4("model", &model);
            gl::DrawArrays(gl::TRIANGLES, 0, 36);
            let model = Mat4::from_translation(vec3(2.0, 0.0, 0.0));
            shader.setMat4("model", &model);
            gl::DrawArrays(gl::TRIANGLES, 0, 36);

            // floor
            gl::BindVertexArray(planeVAO);
            gl::BindTexture(gl::TEXTURE_2D, floorTexture);
            gl::ActiveTexture(gl::TEXTURE0);
            shader.setMat4("model", &Mat4::IDENTITY);
            gl::DrawArrays(gl::TRIANGLES, 0, 6);
            gl::BindVertexArray(0);

            // now draw the mirror quad with screen texture
            // --------------------------------------------
            // disable depth test so screen-space quad isn't discarded due to depth test.
            gl::Disable(gl::DEPTH_TEST);

            screenShader.use_shader();
            gl::BindVertexArray(quadVAO);
            gl::BindTexture(gl::TEXTURE_2D, textureColorbuffer); // use the color attachment texture as the texture of the quad plane
            gl::DrawArrays(gl::TRIANGLES, 0, 6);
        }

        window.swap_buffers();
    }

    // optional: de-allocate all resources once they've outlived their purpose:
    // ------------------------------------------------------------------------
    unsafe {
        gl::DeleteVertexArrays(1, &cubeVAO);
        gl::DeleteVertexArrays(1, &planeVAO);
        gl::DeleteVertexArrays(1, &quadVAO);
        gl::DeleteBuffers(1, &cubeVBO);
        gl::DeleteBuffers(1, &planeVBO);
        gl::DeleteBuffers(1, &quadVAO);
        gl::DeleteRenderbuffers(1, &rbo);
        gl::DeleteFramebuffers(1, &framebuffer);
        gl::DeleteShader(shader.programId);
        gl::DeleteShader(screenShader.programId);
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

        // for this tutorial: use GL_CLAMP_TO_EDGE to prevent semi-transparent borders. Due to interpolation it takes texels from next repeat
        let param = if format == gl::RGBA { gl::CLAMP_TO_EDGE } else { gl::REPEAT };
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, param as GLint);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, param as GLint);

        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR_MIPMAP_LINEAR as GLint);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as GLint);
    }

    texture_id
}
