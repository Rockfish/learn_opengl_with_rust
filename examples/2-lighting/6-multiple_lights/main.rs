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
use image::ColorType;
use learn_opengl_with_rust::camera::{Camera, CameraMovement};
use learn_opengl_with_rust::shader_m::Shader_M;
use learn_opengl_with_rust::SIZE_OF_FLOAT;

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

    // Vertex Array Object id
    let mut cubeVAO: GLuint = 0;
    let mut lightCubeVAO: GLuint = 0;
    // Vertex Buffer Object id
    let mut VBO: GLuint = 0;

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

    // build and compile our shader programs
    // ------------------------------------
    // create shaders
    let mut lightingShader = Shader_M::new();
    lightingShader
        .build(
            "examples/2-lighting/6-multiple_lights/6-multiple_lights.vert",
            "examples/2-lighting/6-multiple_lights/6-multiple_lights.frag",
        )
        .unwrap();

    let mut lightCubeShader = Shader_M::new();
    lightCubeShader
        .build(
            "examples/2-lighting/6-multiple_lights/6-light_cube.vert",
            "examples/2-lighting/6-multiple_lights/6-light_cube.frag",
        )
        .unwrap();

    // set up vertex data (and buffer(s)) and configure vertex attributes
    // ------------------------------------------------------------------
    // vertices needs an explicit type or it will default to f64
    #[rustfmt::skip]
    let vertices: [f32; 288] = [
        // positions       // normals        // texture coords
        -0.5, -0.5, -0.5,  0.0,  0.0, -1.0,  0.0,  0.0,
         0.5, -0.5, -0.5,  0.0,  0.0, -1.0,  1.0,  0.0,
         0.5,  0.5, -0.5,  0.0,  0.0, -1.0,  1.0,  1.0,
         0.5,  0.5, -0.5,  0.0,  0.0, -1.0,  1.0,  1.0,
        -0.5,  0.5, -0.5,  0.0,  0.0, -1.0,  0.0,  1.0,
        -0.5, -0.5, -0.5,  0.0,  0.0, -1.0,  0.0,  0.0,

        -0.5, -0.5,  0.5,  0.0,  0.0,  1.0,  0.0,  0.0,
         0.5, -0.5,  0.5,  0.0,  0.0,  1.0,  1.0,  0.0,
         0.5,  0.5,  0.5,  0.0,  0.0,  1.0,  1.0,  1.0,
         0.5,  0.5,  0.5,  0.0,  0.0,  1.0,  1.0,  1.0,
        -0.5,  0.5,  0.5,  0.0,  0.0,  1.0,  0.0,  1.0,
        -0.5, -0.5,  0.5,  0.0,  0.0,  1.0,  0.0,  0.0,

        -0.5,  0.5,  0.5, -1.0,  0.0,  0.0,  1.0,  0.0,
        -0.5,  0.5, -0.5, -1.0,  0.0,  0.0,  1.0,  1.0,
        -0.5, -0.5, -0.5, -1.0,  0.0,  0.0,  0.0,  1.0,
        -0.5, -0.5, -0.5, -1.0,  0.0,  0.0,  0.0,  1.0,
        -0.5, -0.5,  0.5, -1.0,  0.0,  0.0,  0.0,  0.0,
        -0.5,  0.5,  0.5, -1.0,  0.0,  0.0,  1.0,  0.0,

         0.5,  0.5,  0.5,  1.0,  0.0,  0.0,  1.0,  0.0,
         0.5,  0.5, -0.5,  1.0,  0.0,  0.0,  1.0,  1.0,
         0.5, -0.5, -0.5,  1.0,  0.0,  0.0,  0.0,  1.0,
         0.5, -0.5, -0.5,  1.0,  0.0,  0.0,  0.0,  1.0,
         0.5, -0.5,  0.5,  1.0,  0.0,  0.0,  0.0,  0.0,
         0.5,  0.5,  0.5,  1.0,  0.0,  0.0,  1.0,  0.0,

        -0.5, -0.5, -0.5,  0.0, -1.0,  0.0,  0.0,  1.0,
         0.5, -0.5, -0.5,  0.0, -1.0,  0.0,  1.0,  1.0,
         0.5, -0.5,  0.5,  0.0, -1.0,  0.0,  1.0,  0.0,
         0.5, -0.5,  0.5,  0.0, -1.0,  0.0,  1.0,  0.0,
        -0.5, -0.5,  0.5,  0.0, -1.0,  0.0,  0.0,  0.0,
        -0.5, -0.5, -0.5,  0.0, -1.0,  0.0,  0.0,  1.0,

        -0.5,  0.5, -0.5,  0.0,  1.0,  0.0,  0.0,  1.0,
         0.5,  0.5, -0.5,  0.0,  1.0,  0.0,  1.0,  1.0,
         0.5,  0.5,  0.5,  0.0,  1.0,  0.0,  1.0,  0.0,
         0.5,  0.5,  0.5,  0.0,  1.0,  0.0,  1.0,  0.0,
        -0.5,  0.5,  0.5,  0.0,  1.0,  0.0,  0.0,  0.0,
        -0.5,  0.5, -0.5,  0.0,  1.0,  0.0,  0.0,  1.0,
    ];

    // position all containers
    #[rustfmt::skip]
    let cubePositions: [Vec3; 10] = [
        vec3( 0.0,  0.0,  0.0),
        vec3( 2.0,  5.0, -15.0),
        vec3(-1.5, -2.2, -2.5),
        vec3(-3.8, -2.0, -12.3),
        vec3( 2.4, -0.4, -3.5),
        vec3(-1.7,  3.0, -7.5),
        vec3( 1.3, -2.0, -2.5),
        vec3( 1.5,  2.0, -2.5),
        vec3( 1.5,  0.2, -1.5),
        vec3(-1.3,  1.0, -1.5)
    ];

    // positions of the point lights
    #[rustfmt::skip]
    let pointLightPositions = [
        vec3( 0.7,  0.2,  2.0),
        vec3( 2.3, -3.3, -4.0),
        vec3(-4.0,  2.0, -12.0),
        vec3( 0.0,  0.0, -3.0),
    ];

    unsafe {
        // configure global opengl state
        // -----------------------------
        gl::Enable(gl::DEPTH_TEST);

        // first, configure the cube's VAO (and VBO)
        gl::GenVertexArrays(1, &mut cubeVAO);
        gl::GenBuffers(1, &mut VBO);

        gl::BindBuffer(gl::ARRAY_BUFFER, VBO);
        gl::BufferData(
            gl::ARRAY_BUFFER,
            (vertices.len() * SIZE_OF_FLOAT) as GLsizeiptr,
            vertices.as_ptr() as *const GLvoid,
            gl::STATIC_DRAW,
        );

        gl::BindVertexArray(cubeVAO);

        // position attribute
        gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, 8 * SIZE_OF_FLOAT as GLsizei, 0 as *const GLvoid);
        gl::EnableVertexAttribArray(0);

        // normal attribute
        gl::VertexAttribPointer(
            1,
            3,
            gl::FLOAT,
            gl::FALSE,
            8 * SIZE_OF_FLOAT as GLsizei,
            (3 * SIZE_OF_FLOAT) as *const GLvoid,
        );
        gl::EnableVertexAttribArray(1);

        // texture attribute
        gl::VertexAttribPointer(
            2,
            2,
            gl::FLOAT,
            gl::FALSE,
            8 * SIZE_OF_FLOAT as GLsizei,
            (6 * SIZE_OF_FLOAT) as *const GLvoid,
        );
        gl::EnableVertexAttribArray(2);

        // second, configure the light's VAO (VBO stays the same; the vertices are the
        // same for the light object which is also a 3D cube)
        gl::GenVertexArrays(1, &mut lightCubeVAO);
        gl::BindVertexArray(lightCubeVAO);

        // we only need to bind to the VBO (to link it with glVertexAttribPointer),
        // no need to fill it; the VBO's data already contains all we need
        // (it's already bound, but we do it again for educational purposes)
        gl::BindBuffer(gl::ARRAY_BUFFER, VBO);

        gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, 8 * SIZE_OF_FLOAT as GLsizei, 0 as *const GLvoid);
        gl::EnableVertexAttribArray(0);
    }

    // load textures (we now use a utility function to keep the code more organized)
    // -----------------------------------------------------------------------------
    let diffuseMap = loadTexture("resources/textures/container2.png");
    let specularMap = loadTexture("resources/textures/container2_specular.png");

    // shader configuration
    // --------------------
    lightingShader.use_shader();
    lightingShader.setInt("material.diffuse", 0);
    lightingShader.setInt("material.specular", 1);

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
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT); // also clear the depth buffer now!

            // be sure to activate shader when setting uniforms/drawing objects
            lightingShader.use_shader();
            lightingShader.setVec3("viewPos", &state.camera.Position);
            lightingShader.setFloat("material.shininess", 32.0);

            /*
               Here we set all the uniforms for the 5/6 types of lights we have. We have to set them manually and index
               the proper PointLight struct in the array to set each uniform variable. This can be done more code-friendly
               by defining light types as classes and set their values in there, or by using a more efficient uniform approach
               by using 'Uniform buffer objects', but that is something we'll discuss in the 'Advanced GLSL' tutorial.
            */
            // directional light
            lightingShader.setVec3_xyz("dirLight.direction", -0.2, -1.0, -0.3);
            lightingShader.setVec3_xyz("dirLight.ambient", 0.05, 0.05, 0.05);
            lightingShader.setVec3_xyz("dirLight.diffuse", 0.4, 0.4, 0.4);
            lightingShader.setVec3_xyz("dirLight.specular", 0.5, 0.5, 0.5);
            // point light 1
            lightingShader.setVec3("pointLights[0].position", &pointLightPositions[0]);
            lightingShader.setVec3_xyz("pointLights[0].ambient", 0.05, 0.05, 0.05);
            lightingShader.setVec3_xyz("pointLights[0].diffuse", 0.8, 0.8, 0.8);
            lightingShader.setVec3_xyz("pointLights[0].specular", 1.0, 1.0, 1.0);
            lightingShader.setFloat("pointLights[0].constant", 1.0);
            lightingShader.setFloat("pointLights[0].linear", 0.09);
            lightingShader.setFloat("pointLights[0].quadratic", 0.032);
            // point light 2
            lightingShader.setVec3("pointLights[1].position", &pointLightPositions[1]);
            lightingShader.setVec3_xyz("pointLights[1].ambient", 0.05, 0.05, 0.05);
            lightingShader.setVec3_xyz("pointLights[1].diffuse", 0.8, 0.8, 0.8);
            lightingShader.setVec3_xyz("pointLights[1].specular", 1.0, 1.0, 1.0);
            lightingShader.setFloat("pointLights[1].constant", 1.0);
            lightingShader.setFloat("pointLights[1].linear", 0.09);
            lightingShader.setFloat("pointLights[1].quadratic", 0.032);
            // point light 3
            lightingShader.setVec3("pointLights[2].position", &pointLightPositions[2]);
            lightingShader.setVec3_xyz("pointLights[2].ambient", 0.05, 0.05, 0.05);
            lightingShader.setVec3_xyz("pointLights[2].diffuse", 0.8, 0.8, 0.8);
            lightingShader.setVec3_xyz("pointLights[2].specular", 1.0, 1.0, 1.0);
            lightingShader.setFloat("pointLights[2].constant", 1.0);
            lightingShader.setFloat("pointLights[2].linear", 0.09);
            lightingShader.setFloat("pointLights[2].quadratic", 0.032);
            // point light 4
            lightingShader.setVec3("pointLights[3].position", &pointLightPositions[3]);
            lightingShader.setVec3_xyz("pointLights[3].ambient", 0.05, 0.05, 0.05);
            lightingShader.setVec3_xyz("pointLights[3].diffuse", 0.8, 0.8, 0.8);
            lightingShader.setVec3_xyz("pointLights[3].specular", 1.0, 1.0, 1.0);
            lightingShader.setFloat("pointLights[3].constant", 1.0);
            lightingShader.setFloat("pointLights[3].linear", 0.09);
            lightingShader.setFloat("pointLights[3].quadratic", 0.032);
            // spotLight
            lightingShader.setVec3("spotLight.position", &state.camera.Position);
            lightingShader.setVec3("spotLight.direction", &state.camera.Front);
            lightingShader.setVec3_xyz("spotLight.ambient", 0.0, 0.0, 0.0);
            lightingShader.setVec3_xyz("spotLight.diffuse", 1.0, 1.0, 1.0);
            lightingShader.setVec3_xyz("spotLight.specular", 1.0, 1.0, 1.0);
            lightingShader.setFloat("spotLight.constant", 1.0);
            lightingShader.setFloat("spotLight.linear", 0.09);
            lightingShader.setFloat("spotLight.quadratic", 0.032);
            lightingShader.setFloat("spotLight.cutOff", 12.5f32.to_radians().cos());
            lightingShader.setFloat("spotLight.outerCutOff", 15.0f32.to_radians().cos());

            // view/projection transformations
            let projection = Mat4::perspective_rh_gl(state.camera.Zoom.to_radians(), SCR_WIDTH / SCR_HEIGHT, 0.1, 100.0);
            let view = state.camera.GetViewMatrix();
            lightingShader.setMat4("projection", &projection);
            lightingShader.setMat4("view", &view);

            // world transformation
            let model = Mat4::IDENTITY;
            lightingShader.setMat4("model", &model);

            // bind diffuse map
            gl::ActiveTexture(gl::TEXTURE0);
            gl::BindTexture(gl::TEXTURE_2D, diffuseMap);
            // bind specular map
            gl::ActiveTexture(gl::TEXTURE1);
            gl::BindTexture(gl::TEXTURE_2D, specularMap);

            // render the cube
            // gl::BindVertexArray(cubeVAO);
            // gl::DrawArrays(gl::TRIANGLES, 0, 36);

            gl::BindVertexArray(cubeVAO);
            for (i, cube_pos) in cubePositions.iter().enumerate() {
                let mut model = Mat4::from_translation(*cube_pos);
                let angle = (20.0 * i as f32).to_radians();
                model = model * Mat4::from_axis_angle(Vec3::new(1.0, 0.3, 0.5), angle);
                lightingShader.setMat4("model", &model);

                gl::DrawArrays(gl::TRIANGLES, 0, 36);
            }

            // also draw the lamp objects
            lightCubeShader.use_shader();
            lightCubeShader.setMat4("projection", &projection);
            lightCubeShader.setMat4("view", &view);

            // we now draw as many light bulbs as we have point lights.
            gl::BindVertexArray(lightCubeVAO);
            for pointPos in pointLightPositions {
                let mut model = Mat4::from_translation(pointPos);
                model *= Mat4::from_scale(vec3(0.2, 0.2, 0.2));
                lightCubeShader.setMat4("model", &model);
                gl::DrawArrays(gl::TRIANGLES, 0, 36);
            }
        }

        window.swap_buffers();
    }

    // optional: de-allocate all resources once they've outlived their purpose:
    // ------------------------------------------------------------------------
    unsafe {
        gl::DeleteVertexArrays(2, &cubeVAO);
        gl::DeleteVertexArrays(2, &lightCubeVAO);
        gl::DeleteBuffers(2, &VBO);
        gl::DeleteProgram(lightingShader.programId);
        gl::DeleteProgram(lightCubeShader.programId);
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
