# Camera Exercise 2

## Exercise 2
Try to create your own LookAt function where you manually create a view matrix as discussed
at the start of this chapter. Replace glm’s LookAt function with your own implementation and see if it still acts the same.

### Solution

Manual look at function:

    pub fn calculate_lookAt_matrix(position: Vec3, target: Vec3, worldUp: Vec3) -> Mat4 {
        // 1. Position = known
        // 2. Calculate cameraDirection
        let z_axis = (position - target).normalize();
        // 3. Get positive right axis vector
        let x_axis = worldUp.normalize().cross(z_axis).normalize();
        // 4. Calculate camera up vector
        let y_axis = z_axis.cross(x_axis);

        let mut translation = Mat4::IDENTITY;
        translation.w_axis.x = -position.x;
        translation.w_axis.y = -position.y;
        translation.w_axis.z = -position.z;

        let mut rotation = Mat4::IDENTITY;
        rotation.col_mut(0).x = x_axis.x;
        rotation.col_mut(1).x = x_axis.y;
        rotation.col_mut(2).x = x_axis.z;
        rotation.col_mut(0).y = y_axis.x;
        rotation.col_mut(1).y = y_axis.y;
        rotation.col_mut(2).y = y_axis.z;
        rotation.col_mut(0).z = z_axis.x;
        rotation.col_mut(1).z = z_axis.y;
        rotation.col_mut(2).z = z_axis.z;

        let view = rotation * translation;

        println!("view matrix:\n{}\n", view);

        view
    }

Sample call:

     let view = calculate_lookAt_matrix(
        state.camera.Position,
        state.camera.Position + state.camera.Front,
        state.camera.Up);
