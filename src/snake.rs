use std::collections::VecDeque;
use macroquad::prelude::*;
use crate::{resources::{Resources, self}, body::Body};

pub struct Snake {
    pub x: f32,
    pub y: f32,
    head_up_texture: Texture2D,
    head_down_texture: Texture2D,
    head_left_texture: Texture2D,
    head_right_texture: Texture2D,
    tail_up_texture: Texture2D,
    tail_down_texture: Texture2D,
    tail_left_texture: Texture2D,
    tail_right_texture: Texture2D,
    body_up_texture: Texture2D,
    body_down_texture: Texture2D,
    body_left_texture: Texture2D,
    body_right_texture: Texture2D,
    angel_left_up_texture: Texture2D,
    angel_left_down_texture: Texture2D,
    angel_right_up_texture: Texture2D,
    angel_right_down_texture: Texture2D,
    dir: Dir,
    pub prev_x: f32,
    pub prev_y: f32,
    prev_dir: Dir,
    last_update_time: f64,
    pub body_parts: VecDeque<Body>,
    pub got_food: bool,
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
            head_up_texture: resources.head_up,
            head_down_texture : resources.head_down,
            head_left_texture: resources.head_left,
            head_right_texture: resources.head_right,
            tail_up_texture: resources.tail_up,
            tail_down_texture : resources.tail_down,
            tail_left_texture: resources.tail_left,
            tail_right_texture: resources.tail_right,
            body_up_texture: resources.body_v,
            body_down_texture: resources.body_v,
            body_left_texture: resources.body_h,
            body_right_texture: resources.body_h,
            angel_left_up_texture: resources.angel_left_up,
            angel_left_down_texture: resources.angel_left_down,
            angel_right_up_texture: resources.angel_right_up,
            angel_right_down_texture: resources.angel_right_down,
            dir: Dir::Left,
            prev_x: 300.0,
            prev_y: 300.0,
            prev_dir: Dir::Left,
            last_update_time: 0.0,
            body_parts: VecDeque::new(),
            got_food: false,
        }
    }

    pub fn update(&mut self) {
        if get_time() - self.last_update_time > resources::UPDATE_TIME {
            self.last_update_time = get_time();
        } else {
            return;
        }

        // var for body part name
        let mut part_name: String;

        // previous points
        self.prev_x = self.x;
        self.prev_y = self.y;
        match self.dir {
            Dir::Up => {
                self.prev_dir = Dir::Up;
                if self.body_parts.len() > 1 {
                    part_name = "body_up".to_string()
                } else {
                    part_name = "tail_up".to_string()
                }
            },
            Dir::Down => {
                self.prev_dir = Dir::Down;
                if self.body_parts.len() > 1 {
                    part_name = "body_down".to_string()
                } else {
                    part_name = "tail_down".to_string()
                }
            },
            Dir::Left => {
                self.prev_dir = Dir::Left;
                if self.body_parts.len() > 1 {
                    part_name = "body_left".to_string()
                } else {
                    part_name = "tail_left".to_string()
                }
            },
            Dir::Right => {
                self.prev_dir = Dir::Right;
                if self.body_parts.len() > 1 {
                    part_name = "body_right".to_string()
                } else {
                    part_name = "tail_right".to_string()
                }
            },
        }

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

        if self.body_parts.len() > 1 {
            if self.prev_dir == Dir::Right && self.dir == Dir::Down {
                part_name = "angel_left_down".to_string();
            }
            if self.prev_dir == Dir::Right && self.dir == Dir::Up {
                part_name = "angel_left_up".to_string();
            }
            if self.prev_dir == Dir::Left && self.dir == Dir::Down {
                part_name = "angel_right_down".to_string();
            }
            if self.prev_dir == Dir::Left && self.dir == Dir::Up {
                part_name = "angel_right_up".to_string();
            }
            if self.prev_dir == Dir::Up && self.dir == Dir::Left {
                part_name = "angel_left_down".to_string();
            }
            if self.prev_dir == Dir::Up && self.dir == Dir::Right {
                part_name = "angel_right_down".to_string();
            }
            if self.prev_dir == Dir::Down && self.dir == Dir::Left {
                part_name = "angel_left_up".to_string();
            }
            if self.prev_dir == Dir::Down && self.dir == Dir::Right {
                part_name = "angel_right_up".to_string();
            }
        }

        self.body_parts.push_front(
            Body::new(self.prev_x, self.prev_y, part_name),
        );
        if !self.got_food {
            self.body_parts.pop_back();
        } else {
            self.got_food = false;
        }

        match self.dir {
            Dir::Up => {
                self.y -= resources::BLOCKSIZE;
                if self.y < 0.0 {
                    self.y = resources::RES_WIDTH as f32 - resources::BLOCKSIZE;
                }
            },
            Dir::Down => {
                self.y += resources::BLOCKSIZE;
                if self.y > resources::RES_WIDTH as f32 - resources::BLOCKSIZE {
                    self.y = 0.0
                }
            },
            Dir::Left => {
                self.x -= resources::BLOCKSIZE;
                if self.x < 0.0 {
                    self.x = resources::RES_WIDTH as f32 - resources::BLOCKSIZE;
                }
            },
            Dir::Right => {
                self.x += resources::BLOCKSIZE;
                if self.x > resources::RES_WIDTH as f32 - resources::BLOCKSIZE {
                    self.x = 0.0
                }
            },
        }
    }

    pub fn draw(&mut self) {
        match self.dir {
            Dir::Up => {draw_texture(self.head_up_texture, self.x, self.y, WHITE);},
            Dir::Down => {draw_texture(self.head_down_texture, self.x, self.y, WHITE);},
            Dir::Left => {draw_texture(self.head_left_texture, self.x, self.y, WHITE);},
            Dir::Right => {draw_texture(self.head_right_texture, self.x, self.y, WHITE);},
        }

        for part in &mut self.body_parts {
            match part.value.to_string().as_str() {
                "tail_left" => {draw_texture(self.tail_left_texture, part.x, part.y, WHITE);},
                "tail_right" => {draw_texture(self.tail_right_texture, part.x, part.y, WHITE);},
                "tail_up" => {draw_texture(self.tail_up_texture, part.x, part.y, WHITE);},
                "tail_down" => {draw_texture(self.tail_down_texture, part.x, part.y, WHITE);},
                "body_left" => {draw_texture(self.body_left_texture, part.x, part.y, WHITE);},
                "body_right" => {draw_texture(self.body_right_texture, part.x, part.y, WHITE);},
                "body_up" => {draw_texture(self.body_up_texture, part.x, part.y, WHITE);},
                "body_down" => {draw_texture(self.body_down_texture, part.x, part.y, WHITE);},
                "angel_left_up" => {draw_texture(self.angel_left_up_texture, part.x, part.y, WHITE);},
                "angel_left_down" => {draw_texture(self.angel_left_down_texture, part.x, part.y, WHITE);},
                "angel_right_up" => {draw_texture(self.angel_right_up_texture, part.x, part.y, WHITE);},
                "angel_right_down" => {draw_texture(self.angel_right_down_texture, part.x, part.y, WHITE);},
                _ => {}
            }
        }
        
        if self.body_parts.len() > 1 {
            let last_index = self.body_parts.len() - 1;
            match self.body_parts.back().unwrap().value.to_string().as_str() {
                "body_up" => {draw_texture(self.tail_up_texture, self.body_parts[last_index].x, self.body_parts[last_index].y, WHITE);},
                "body_down" => {draw_texture(self.tail_down_texture, self.body_parts[last_index].x, self.body_parts[last_index].y, WHITE);},
                "body_left" => {draw_texture(self.tail_left_texture, self.body_parts[last_index].x, self.body_parts[last_index].y, WHITE);},
                "body_right" => {draw_texture(self.tail_right_texture, self.body_parts[last_index].x, self.body_parts[last_index].y, WHITE);},
                _ => {},
            };
        }
    }
}
