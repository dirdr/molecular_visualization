trait SceneMove {
    fn move_forward(&mut self);
    fn move_backward(&mut self);
    fn strafe(&mut self, direction: StrafeDirection);
    // TODO - add rotate (yaw pitch etc...)
}

enum StrafeDirection {
    Left,
    Right,
}
