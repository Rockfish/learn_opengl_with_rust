# Transformations Exercise 2

## Exercise 2
Try to make every 3rd container (including the 1st) rotate over time, while leaving the other
containers static using just the model matrix.

### Solution

Add an enumeration and use the index to change the model for every third cube:

    int main()
    {
        [...]
        while(!glfwWindowShouldClose(window))
        {
            [...]        

            for (i, cube_pos) in cubePositions.iter().enumerate() {
                let mut model = Mat4::IDENTITY;
                model = model * Mat4::from_translation(*cube_pos);

                // every 3rd iteration (including the first) we set the angle using GLFW's time function.
                let angle = if i % 3 == 0 {
                    (glfw.get_time() * 25.0).to_radians() as f32
                } else {
                    (20.0 * i as f32).to_radians() as f32
                };

                model = model * Mat4::from_axis_angle(Vec3::new(1.0, 0.3, 0.5), angle);
                ourShader.setMat4("model", &model);

                gl::DrawArrays(gl::TRIANGLES, 0, 36);
            }

            [...]        
        }
    }


