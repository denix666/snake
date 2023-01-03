use macroquad::prelude::*;

// window size in pixels
pub const RES_WIDTH: i32 = 600;
pub const RES_HEIGHT: i32 = 600;

pub const BLOCKSIZE: f32 = 30.0;
pub const UPDATE_TIME: f64 = 0.1;

pub struct Resources {
    pub grid: Texture2D,
    pub bg: Texture2D,
    pub head_up: Texture2D,
    pub head_down: Texture2D,
    pub head_left: Texture2D,
    pub head_right: Texture2D,
    pub tail_up: Texture2D,
    pub tail_down: Texture2D,
    pub tail_left: Texture2D,
    pub tail_right: Texture2D,
    pub body_h: Texture2D,
    pub body_v: Texture2D,
    pub angel_left_down: Texture2D,
    pub angel_left_up: Texture2D,
    pub angel_right_down: Texture2D,
    pub angel_right_up: Texture2D,
    pub apple: Texture2D,
}

impl Resources {
    pub async fn new() -> Self {
        Self {
            grid: load_texture("assets/images/grid.png").await.unwrap(),
            bg: load_texture("assets/images/bg.png").await.unwrap(),
            head_up: load_texture("assets/images/head_up.png").await.unwrap(),
            head_down: load_texture("assets/images/head_down.png").await.unwrap(),
            head_left: load_texture("assets/images/head_left.png").await.unwrap(),
            head_right: load_texture("assets/images/head_right.png").await.unwrap(),
            tail_up: load_texture("assets/images/tail_up.png").await.unwrap(),
            tail_down: load_texture("assets/images/tail_down.png").await.unwrap(),
            tail_left: load_texture("assets/images/tail_left.png").await.unwrap(),
            tail_right: load_texture("assets/images/tail_right.png").await.unwrap(),
            body_h: load_texture("assets/images/body_h.png").await.unwrap(),
            body_v: load_texture("assets/images/body_v.png").await.unwrap(),
            angel_left_down: load_texture("assets/images/angel_left_down.png").await.unwrap(),
            angel_left_up: load_texture("assets/images/angel_left_up.png").await.unwrap(),
            angel_right_down: load_texture("assets/images/angel_right_down.png").await.unwrap(),
            angel_right_up: load_texture("assets/images/angel_right_up.png").await.unwrap(),
            apple: load_texture("assets/images/apple.png").await.unwrap(),
        }
    }
}
