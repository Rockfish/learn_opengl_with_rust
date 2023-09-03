#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(unused_assignments)]
#![allow(clippy::zero_ptr)]
#![allow(clippy::assign_op_pattern)]

extern crate glfw;

use glad_gl::gl;
use glad_gl::gl::{GLint, GLsizei, GLsizeiptr, GLuint, GLvoid};
use glam::*;
use glfw::{Action, Context, Key};
use learn_opengl_with_rust::shader_m::Shader_M;
use learn_opengl_with_rust::SIZE_OF_FLOAT;

const SCR_WIDTH: u32 = 800;
const SCR_HEIGHT: u32 = 800;

// Struct for passing state between the window loop and the event handler.
struct State {
    cameraPos: Vec3,
    cameraFront: Vec3,
    cameraUp: Vec3,
    deltaTime: f32,
    lastFrame: f32,
    firstMouse: bool,
    yaw: f32,
    pitch: f32,
    lastX: f32,
    lastY: f32,
    fov: f32,
}

fn main() {
    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();

    glfw.window_hint(glfw::WindowHint::ContextVersion(3, 3));
    glfw.window_hint(glfw::WindowHint::OpenGlProfile(glfw::OpenGlProfileHint::Core));

    // for Apple
    glfw.window_hint(glfw::WindowHint::OpenGlForwardCompat(true));

    let (mut window, events) = glfw
        .create_window(SCR_WIDTH, SCR_HEIGHT, "LearnOpenGL", glfw::WindowMode::Windowed)
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
    // Texture ids
    let mut texture1: GLuint = 0;
    let mut texture2: GLuint = 0;

    // build and compile our shader program
    let ourShader = Shader_M::new(
        "examples/1-getting_started/7_3-camera_mouse_zoom/7_3-camera.vert",
        "examples/1-getting_started/7_3-camera_mouse_zoom/7_3-camera.frag",
    )
    .unwrap();

    // set up vertex data (and buffer(s)) and configure vertex attributes
    // ------------------------------------------------------------------
    #[rustfmt::skip]
    let vertices: [f32; 180] = [
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
        -0.5,  0.5, -0.5,  0.0, 1.0,
    ];

    // world space positions of our cubes
    #[rustfmt::skip]
    let cubePositions: [Vec3; 10] = [
        Vec3::new( 0.0,  0.0,  0.0),
        Vec3::new( 2.0,  5.0, -15.0),
        Vec3::new(-1.5, -2.2, -2.5),
        Vec3::new(-3.8, -2.0, -12.3),
        Vec3::new( 2.4, -0.4, -3.5),
        Vec3::new(-1.7,  3.0, -7.5),
        Vec3::new( 1.3, -2.0, -2.5),
        Vec3::new( 1.5,  2.0, -2.5),
        Vec3::new( 1.5,  0.2, -1.5),
        Vec3::new(-1.3,  1.0, -1.5)
    ];

    unsafe {
        gl::Enable(gl::DEPTH_TEST);

        gl::GenVertexArrays(1, &mut VAO);
        gl::GenBuffers(1, &mut VBO);

        gl::BindVertexArray(VAO);

        gl::BindBuffer(gl::ARRAY_BUFFER, VBO);
        gl::BufferData(
            gl::ARRAY_BUFFER,
            (vertices.len() * SIZE_OF_FLOAT) as GLsizeiptr,
            vertices.as_ptr() as *const GLvoid,
            gl::STATIC_DRAW,
        );

        // position attribute
        gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, 5 * SIZE_OF_FLOAT as GLsizei, 0 as *const GLvoid);
        gl::EnableVertexAttribArray(0);

        // texture coordinate attribute
        gl::VertexAttribPointer(
            1,
            2,
            gl::FLOAT,
            gl::FALSE,
            5 * SIZE_OF_FLOAT as GLsizei,
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
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR_MIPMAP_LINEAR as GLint);
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
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as GLint);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as GLint);

        // load image, create texture and generate mipmaps
        let img = image::open("resources/textures/awesomeface.png").expect("Texture failed to load");
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

    // Initialize the world state
    let mut state = State {
        cameraPos: vec3(0.0, 0.0, 3.0),
        cameraFront: vec3(0.0, 0.0, -1.0),
        cameraUp: vec3(0.0, 1.0, 0.0),
        deltaTime: 0.0,
        lastFrame: 0.0,
        firstMouse: true,
        // yaw is initialized to -90.0 degrees since a yaw of 0.0 results in a direction vector
        // pointing to the right so we initially rotate a bit to the left.
        yaw: -90.0,
        pitch: 0.0,
        lastX: SCR_WIDTH as f32 / 2.0,
        lastY: SCR_HEIGHT as f32 / 2.0,
        fov: 45.0,
    };

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
            gl::ClearColor(0.2, 0.3, 0.3, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT); // also clear the depth buffer now!

            // bind textures on corresponding texture units
            gl::ActiveTexture(gl::TEXTURE0);
            gl::BindTexture(gl::TEXTURE_2D, texture1);
            gl::ActiveTexture(gl::TEXTURE1);
            gl::BindTexture(gl::TEXTURE_2D, texture2);

            // activate shader
            ourShader.use_shader();

            // pass projection matrix to shader (as projection matrix rarely changes there's no need to do this per frame)
            // -----------------------------------------------------------------------------------------------------------
            let projection = Mat4::perspective_rh_gl(state.fov.to_radians(), (SCR_WIDTH / SCR_HEIGHT) as f32, 0.1, 100.0);
            ourShader.setMat4("projection", &projection);

            // OpenGL uses a right handed coordinate system so use *_rh methods.
            // eye: Position of the camera
            // center: Position where the camera is looking at
            // up: Normalized up vector, how the camera is oriented.
            let view = Mat4::look_at_rh(state.cameraPos, state.cameraPos + state.cameraFront, state.cameraUp);

            ourShader.setMat4("view", &view);

            // render boxes
            gl::BindVertexArray(VAO);

            for (i, cube_pos) in cubePositions.iter().enumerate() {
                let mut model = Mat4::from_translation(*cube_pos);
                let angle = (20.0 * i as f32).to_radians();
                model = model * Mat4::from_axis_angle(Vec3::new(1.0, 0.3, 0.5), angle);
                ourShader.setMat4("model", &model);

                gl::DrawArrays(gl::TRIANGLES, 0, 36);
            }
        }

        window.swap_buffers();
    }

    // optional: de-allocate all resources once they've outlived their purpose:
    // ------------------------------------------------------------------------
    unsafe {
        gl::DeleteVertexArrays(2, &VAO);
        gl::DeleteBuffers(2, &VBO);
        gl::DeleteProgram(ourShader.programId);
    }
}

//
// GLFW maps callbacks to events.
//
fn handle_window_event(window: &mut glfw::Window, event: glfw::WindowEvent, state: &mut State) {
    let cameraSpeed = state.deltaTime * 2.5;

    match event {
        glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => window.set_should_close(true),
        glfw::WindowEvent::FramebufferSize(width, height) => {
            framebuffer_size_event(window, width, height);
        }
        glfw::WindowEvent::Key(Key::W, _, _, _) => {
            state.cameraPos += cameraSpeed * state.cameraFront;
        }
        glfw::WindowEvent::Key(Key::S, _, _, _) => {
            state.cameraPos -= cameraSpeed * state.cameraFront;
        }
        glfw::WindowEvent::Key(Key::A, _, _, _) => {
            state.cameraPos -= state.cameraFront.cross(state.cameraUp).normalize() * cameraSpeed;
        }
        glfw::WindowEvent::Key(Key::D, _, _, _) => {
            state.cameraPos += state.cameraFront.cross(state.cameraUp).normalize() * cameraSpeed;
        }
        glfw::WindowEvent::CursorPos(xpos, ypos) => mouse_handler(state, xpos, ypos),
        glfw::WindowEvent::Scroll(xoffset, ysoffset) => scroll_handler(state, xoffset, ysoffset),
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

fn mouse_handler(state: &mut State, xposIn: f64, yposIn: f64) {
    let xpos = xposIn as f32;
    let ypos = yposIn as f32;

    if state.firstMouse {
        state.lastX = xpos;
        state.lastY = ypos;
        state.firstMouse = false;
    }

    let mut xoffset = xpos - state.lastX;
    let mut yoffset = state.lastY - ypos; // reversed since y-coordinates go from bottom to top
    state.lastX = xpos;
    state.lastY = ypos;

    let sensitivity: f32 = 0.1; // change this value to your liking
    xoffset *= sensitivity;
    yoffset *= sensitivity;

    state.yaw += xoffset;
    state.pitch += yoffset;

    // make sure that when pitch is out of bounds, screen doesn't get flipped
    if state.pitch > 89.0 {
        state.pitch = 89.0;
    }
    if state.pitch < -89.0 {
        state.pitch = -89.0;
    }

    let front = vec3(
        state.yaw.to_radians().cos() * state.pitch.to_radians().cos(),
        state.pitch.to_radians().sin(),
        state.yaw.to_radians().sin() * state.pitch.to_radians().cos(),
    );

    state.cameraFront = front.normalize();
}

fn scroll_handler(state: &mut State, _xoffset: f64, yoffset: f64) {
    state.fov -= yoffset as f32;
    if state.fov < 1.0 {
        state.fov = 1.0
    }
    if state.fov > 45.0 {
        state.fov = 45.0
    }
}
