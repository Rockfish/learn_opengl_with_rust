# Transformations Exercise 1

## Exercise 1
Using the last transformation on the container, try switching the order around by 
first rotating and then translating. 
See what happens and try to reason why this happens

### Solution

Make the following changes to the Rust code:

    int main()
    {
        [...]
        while(!glfwWindowShouldClose(window))
        {
            [...]        

            // create transformations using glam
            let mut transform = Mat4::IDENTITY;
            transform = transform * Mat4::from_axis_angle(Vec3::new(0.0, 0.0, 1.0), glfw.get_time() as f32);
            transform = transform * Mat4::from_translation(Vec3::new(0.5, -0.5, 0.0));

            [...]        
        }
    }

### Why does our container now spin around our screen?:
Remember that matrix multiplication is applied in reverse. This time a translation is thus
applied first to the container positioning it in the bottom-right corner of the screen.
After the translation the rotation is applied to the translated container.

A rotation transformation is also known as a change-of-basis transformation
for when we dig a bit deeper into linear algebra. Since we're changing the
basis of the container, the next resulting translations will translate the container
based on the new basis vectors. Once the vector is slightly rotated, the vertical
translations would also be slightly translated for example.

If we would first apply rotations then they'd resolve around the rotation origin (0,0,0), but
since the container is first translated, its rotation origin is no longer (0,0,0) making it
looks as if it's circling around the origin of the scene.

If you had trouble visualizing this or figuring it out, don't worry. If you
experiment with transformations you'll soon get the grasp of it; all it takes
is practice and experience.

