# Camera Exercise 1

## Exercise 1
See if you can transform the camera class in such away that it becomes a true fps camera
where you cannot fly; you can only look around while staying on the xz plane.

### Solution

This function is found in the camera class. What we basically do is keep 
the y position value at 0.0f to force our user to stick to the ground.

    // processes input received from any keyboard-like input system. Accepts input parameter
    // in the form of camera defined ENUM (to abstract it from windowing systems)
    pub fn ProcessKeyboard(&mut self, direction: CameraMovement, deltaTime: f32) {
        let velocity: f32 = self.MovementSpeed * deltaTime;

        match direction {
            CameraMovement::FORWARD => self.Position += self.Front * velocity,
            CameraMovement::BACKWARD => self.Position -= self.Front * velocity,
            CameraMovement::LEFT => self.Position -= self.Right * velocity,
            CameraMovement::RIGHT => self.Position += self.Right * velocity,
        }

        // For FPS: make sure the user stays at the ground level
        self.Position.y = 0.0; // <-- this one-liner keeps the user at the ground level (xz plane)
    }


