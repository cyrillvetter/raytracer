pub struct Camera {
    pub size: f32
}

impl Camera {
    pub const fn new(size: f32) -> Self {
        Self { size }
    }
}
