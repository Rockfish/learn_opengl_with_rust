#version 330 core
layout (location = 0) in vec2 aPos;
layout (location = 1) in vec2 aTexCoords;

out vec2 TexCoords;

void main()
{
    TexCoords = aTexCoords;
    // reduce size and move to upper right corner
    gl_Position = vec4(aPos.x * 0.3f + 0.7f, aPos.y * 0.3f + 0.7f, 0.0, 1.0);
}  