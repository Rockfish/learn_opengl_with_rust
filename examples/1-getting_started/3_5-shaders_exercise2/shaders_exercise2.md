# Shaders Exercise 2

## Exercise 2
Specify a horizontal offset via a uniform and move the triangle to the right side of the screen
in the vertex shader using this offset value.

### Solution

Make the following changes to the Rust file:

    // In your Rust file:
    // ======================
    let offset: f32 = 0.5;
    ourShader.setFloat("xOffset", offset);

Make the following change in the vertex file:

    // In your vertex shader:
    // ======================
    #version 330 core
    layout (location = 0) in vec3 aPos;
    layout (location = 1) in vec3 aColor;

    out vec3 ourColor;

    uniform float xOffset;

    void main()
    {
        // add the xOffset to the x position of the vertex position
        gl_Position = vec4(aPos.x + xOffset, aPos.y, aPos.z, 1.0); 
        ourColor = aColor;
    }
