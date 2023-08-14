# Shaders Exercise 1

## Exercise 1
Adjust the vertex shader so that the triangle is upside down. 

### Solution

Make the following changes to the vertex shader:

    #version 330 core
    layout (location = 0) in vec3 aPos;
    layout (location = 1) in vec3 aColor;

    out vec3 ourColor;

    void main()
    {
        gl_Position = vec4(aPos.x, -aPos.y, aPos.z, 1.0); // just add a - to the y position
        ourColor = aColor;
    }

