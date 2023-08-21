# Lighting Maps Exercise 4

## Exercise

Also add something they call an emission map which is a texture that stores emission values 
per fragment. Emission values are colors an object may emit as if it contains a light source 
itself; this way an object can glow regardless of the light conditions. Emission maps are often 
what you see when objects in a game glow (like the eyes of a robot, or light strips on a container). 
Add the following texture (by creativesam) as an emission map onto the container as if the letters 
emit light: learnopengl.com/img/textures/matrix.jpg.

## Solution

Added to main.rs

    let emissionMap = loadTexture("resources/textures/matrix.jpg");
    lightingShader.setInt("material.emission", 2);

In the loop: 

     // bind emission map
     gl::ActiveTexture(gl::TEXTURE2);
     gl::BindTexture(gl::TEXTURE_2D, emissionMap);
