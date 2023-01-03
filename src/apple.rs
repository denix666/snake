use macroquad::prelude::*;

use crate::resources::Resources;

pub struct Apple {
    pub x: f32,
    pub y: f32, 
    pub destroyed: bool,
    texture: Texture2D,
    pub rect: Rect,
}

impl Apple {
    pub async fn new(x:f32, y:f32, resources: &Resources)  -> Self {
        Self {
            x,
            y,
            destroyed: false,
            texture: resources.apple,
            rect: Rect::new(0.0, 0.0, 30.0,30.0),
        }
    }

    pub fn draw(&mut self) {
        draw_texture(self.texture, self.x, self.y, WHITE);

        // define rect
        self.rect.x = self.x + 30.0;
        self.rect.y = self.y + 30.0;
    }
}
