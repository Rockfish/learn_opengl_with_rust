# Shaders Exercise 1

## Exercise 1
Make sure only the happy face looks in the other/reverse direction by changing the fragment shader

### Solution

Make the following changes to the fragment shader:

    #version 330 core
    out vec4 FragColor;
    
    in vec3 ourColor;
    in vec2 TexCoord;
    
    uniform sampler2D ourTexture1;
    uniform sampler2D ourTexture2;
    
    void main()
    {
        FragColor = mix(texture(ourTexture1, TexCoord), texture(ourTexture2, vec2(1.0 - TexCoord.x, TexCoord.y)), 0.2);
    }