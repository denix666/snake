#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

extern crate rand;

use rand::{Rng};

use macroquad::prelude::*;

mod resources;
use resources::Resources;

mod snake;
use snake::Snake;

mod apple;
use apple::Apple;

mod body;
use body::Body;

fn window_conf() -> Conf {
    let mut title = String::from("Snake v");
    title.push_str(env!("CARGO_PKG_VERSION"));
    Conf {
        window_title: title
        .to_owned(),
        fullscreen: false,
        sample_count: 16,
        window_width: resources::RES_WIDTH,
        window_height: resources::RES_HEIGHT,
        ..Default::default()
    }
}

fn draw_map(resources: &Resources) {
    draw_texture(resources.bg,0.0,0.0,WHITE);
    draw_texture(resources.grid,0.0,0.0,WHITE);
}

pub enum GameState {
    Intro,
    InitLevel,
    Game,
    LevelFail,
    Paused,
}

#[macroquad::main(window_conf)]
async fn main() {
    let resources = Resources::new().await;
    let mut game_state = GameState::Intro;
    let mut snake = Snake::new(&resources).await;
    let mut apples: Vec<Apple> = Vec::new();

    loop {
        clear_background(BLACK);
        
        match game_state {
            GameState::Intro=>{
                game_state = GameState::InitLevel;
            },
            GameState::InitLevel => {
                apples.push(
                    Apple::new(90.0, 300.0, &resources).await,
                );
                snake.body_parts.push_front(
                    Body::new(snake.prev_x, snake.prev_y, "tail_right".to_string()),
                );
                game_state = GameState::Game;
            },
            GameState::Game => {
                draw_map(&resources);

                snake.update();

                for apple in &mut apples {
                    apple.draw();

                    if apple.x == snake.x && apple.y == snake.y {
                        apple.destroyed = true;
                        snake.got_food = true;
                    }
                }

                if apples.len() == 0 {
                    let x = rand::thread_rng().gen_range(2..=18);
                    let y = rand::thread_rng().gen_range(2..=18);
                    apples.push(
                        Apple::new(x as f32 * resources::BLOCKSIZE, y as f32 * resources::BLOCKSIZE, &resources).await,
                    );
                }

                snake.draw();
            },
            GameState::LevelFail => {

            },
            GameState::Paused => {

            }, 
        }

        match apples.iter().position(|x| x.destroyed == true) {
            Some(idx) => {
                apples.remove(idx);
            },
            None => {},
        };

        next_frame().await
    }
}