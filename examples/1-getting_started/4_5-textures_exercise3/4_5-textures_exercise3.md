# Textures Exercise 3

## Exercise 3
Try to display only the center pixels of the texture image on the rectangle in 
such a way that the individual pixels are getting visible by changing the texture coordinates. 
Try to set the texture filtering method to GL_NEAREST to see the pixels more clearly.

### Solution

In the Rust file, make the following changes.

Change the texture coordinates to a small area in the middle of the texture.

    let vertices: [f32; 32] = [
        // positions      // colors        // texture coordinates for the center of the textures
        0.5,  0.5, 0.0,   1.0, 0.0, 0.0,   0.55, 0.55, // top right
        0.5, -0.5, 0.0,   0.0, 1.0, 0.0,   0.55, 0.45, // bottom right
       -0.5, -0.5, 0.0,   0.0, 0.0, 1.0,   0.45, 0.45, // bottom left
       -0.5,  0.5, 0.0,   1.0, 1.0, 0.0,   0.45, 0.55  // top left
    ];

Change the texture parameter to NEAREST.

    gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::NEAREST as GLint);