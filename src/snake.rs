use std::{thread, time::Duration};

use macroquad::prelude::*;
use crate::resources::{Resources};

pub struct Snake {
    pub x: f32,
    pub y: f32,
    up_texture: Texture2D,
    down_texture: Texture2D,
    left_texture: Texture2D,
    right_texture: Texture2D,
    dir: Dir,
}

#[derive(PartialEq)]
enum Dir {
    Up,
    Down,
    Left,
    Right,
}

impl Snake {
    pub async fn new(resources: &Resources) -> Self {        
        Self {
            x: 300.0,
            y: 300.0,
            up_texture: resources.head_up,
            down_texture : resources.head_down,
            left_texture: resources.head_left,
            right_texture: resources.head_right,
            dir: Dir::Left,
        }
    }

    pub fn update(&mut self) {
        thread::sleep(Duration::from_millis(crate::resources::THREAD_SLEEP));
        
        if is_key_down(KeyCode::Left) {
            if self.dir != Dir::Left && self.dir != Dir::Right {
                self.dir = Dir::Left;
            }
        }

        if is_key_down(KeyCode::Right) && self.dir != Dir::Left {
            if self.dir != Dir::Right {
                self.dir = Dir::Right;
            }
        }

        if is_key_down(KeyCode::Up) && self.dir != Dir::Down {
            if self.dir != Dir::Up {
                self.dir = Dir::Up;
            }
        }

        if is_key_down(KeyCode::Down) && self.dir != Dir::Up {
            if self.dir != Dir::Down {
                self.dir = Dir::Down;
            }
        }

        match self.dir {
            Dir::Up => {
                self.y -= 30.0;
                if self.y < 0.0 {
                    self.y = 570.0
                }
            },
            Dir::Down => {
                self.y += 30.0;
                if self.y > 570.0 {
                    self.y = 0.0
                }
            },
            Dir::Left => {
                self.x -= 30.0;
                if self.x < 0.0 {
                    self.x = 570.0
                }
            },
            Dir::Right => {
                self.x += 30.0;
                if self.x > 570.0 {
                    self.x = 0.0
                }
            },
        }
    }

    pub fn draw(&mut self) {
        match self.dir {
            Dir::Up => {
                draw_texture(self.up_texture, self.x, self.y, WHITE);
            },
            Dir::Down => {
                draw_texture(self.down_texture, self.x, self.y, WHITE);
            },
            Dir::Left => {
                draw_texture(self.left_texture, self.x, self.y, WHITE);
            },
            Dir::Right => {
                draw_texture(self.right_texture, self.x, self.y, WHITE);
            },
        }
    }
}
