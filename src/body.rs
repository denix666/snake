pub struct Body {
    pub x: f32,
    pub y: f32,
    pub value: String,
}

impl Body {
    pub fn new(x: f32, y: f32, value: String) -> Self {
        Self {
            x,
            y,
            value,
        }
    }
}
