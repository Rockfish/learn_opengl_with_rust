# Shaders Exercise 3

## Exercise 3
Output the vertex position to the fragment shader using the out keyword and set the 
fragment’s color equal to this vertex position (see how even the vertex position values are 
interpolated across the triangle). Once you managed to do this, try to answer the 
following question: why is the bottom-left side of our triangle black?

### Solution

Make the following changes to the vertex shader:

    // Vertex shader:
    // ==============
    #version 330 core
    layout (location = 0) in vec3 aPos;
    layout (location = 1) in vec3 aColor;

    // out vec3 ourColor;
    out vec3 ourPosition;

    void main()
    {
        gl_Position = vec4(aPos, 1.0);
        // ourColor = aColor;
        ourPosition = aPos;
    }

Make the following changes to the fragment shader:
    
    // Fragment shader:
    // ================
    #version 330 core
    out vec4 FragColor;
    // in vec3 ourColor;
    in vec3 ourPosition;
    
    void main()
    {
        // note how the position value is linearly interpolated to get all the different colors
        FragColor = vec4(ourPosition, 1.0);
    }


### Answer to the question: Do you know why the bottom-left side is black?

Think about this for a second: the output of our fragment's color is equal to the (interpolated) coordinate of
the triangle. What is the coordinate of the bottom-left point of our triangle? This is (-0.5f, -0.5f, 0.0f). Since the
xy values are negative they are clamped to a value of 0.0f. This happens all the way to the center sides of the
triangle since from that point on the values will be interpolated positively again. Values of 0.0f are of course black
and that explains the black side of the triangle.
