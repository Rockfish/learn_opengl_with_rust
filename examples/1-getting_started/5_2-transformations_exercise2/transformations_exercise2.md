# Transformations Exercise 2

## Exercise 2
Try drawing a second container with another call to glDrawElements but place it at a different position 
using transformations only. Make sure this second container is placed at the top-left of the window and 
instead of rotating, scale it over time (using the sin function is useful here; note that using sin will 
cause the object to invert as soon as a negative scale is applied).

### Solution

Added a second translation and draw command:

    int main()
    {
        [...]
        while(!glfwWindowShouldClose(window))
        {
            [...]        

            // second transformation
            let mut transform = Mat4::IDENTITY;
            transform = transform * Mat4::from_translation(Vec3::new(-0.5, 0.5, 0.0));
            let scaleAmount = glfw.get_time().sin() as f32;
            transform = transform * Mat4::from_scale(Vec3::new(scaleAmount, scaleAmount, scaleAmount));

            // get matrix's uniform location and set matrix
            let c_str = c_string!("transform");
            let transformLoc = gl::GetUniformLocation(ourShader.programId, c_str.as_ptr());
            gl::UniformMatrix4fv(
                transformLoc,
                1,
                gl::FALSE,
                transform.to_cols_array().as_ptr(),
            );

            // now with the uniform matrix being replaced with new transformations, draw it again.
            gl::DrawElements(gl::TRIANGLES, 6, gl::UNSIGNED_INT, 0 as *const GLvoid);

            [...]        
        }
    }


