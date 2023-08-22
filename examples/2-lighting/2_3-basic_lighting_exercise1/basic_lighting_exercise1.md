# Basic Lighting Exercise 1

## Exercise 1
Right now the light source is a boring static light source that does not move. Try to move
the light source around the scene over time using either sin or cos. Watching the lighting 
change over time gives you a good understanding of Phong’s lighting model.

### Solution

    // render loop
    while !window.should_close() {
        [...]

        // For Exercise 1
        // change the light's position values over time (can be done anywhere in the render loop actually,
        // but try to do it at least before using the light source positions)
        state.lightPos.x = 1.0 + (glfw.get_time() as f32).sin() * 2.0;
        state.lightPos.y = (glfw.get_time() / 2.0).sin() as f32 * 1.0;

        [...]
    }


