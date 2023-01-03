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

fn show_text(font: Font, header_text: &str, message_text: &str) {
    let header_dims = measure_text(header_text, Some(font), 70, 1.0);
    let message_dims = measure_text(message_text, Some(font), 20, 1.0);

    draw_text_ex(
        &header_text,
        screen_width() * 0.5 - header_dims.width * 0.5,
        240.0,
        TextParams {
            font,
            font_size: 70,
            color: ORANGE,
            ..Default::default()
        },
    );

    draw_text_ex(
        &message_text,
        screen_width() * 0.5 - message_dims.width * 0.5,
        280.0,
        TextParams {
            font,
            font_size: 20,
            color: ORANGE,
            ..Default::default()
        },
    );
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
                apples.clear();
                snake.body_parts.clear();
                snake = Snake::new(&resources).await;
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

                for part in &mut snake.body_parts {
                    if part.x == snake.x && part.y == snake.y {
                        game_state = GameState::LevelFail;
                    }
                }

                for apple in &mut apples {
                    apple.draw();

                    if apple.x == snake.x && apple.y == snake.y {
                        apple.destroyed = true;
                        snake.got_food = true;
                    }
                }

                if apples.len() == 0 {
                    let mut item_placed_successfully: bool = false;
                    while !item_placed_successfully {
                        let mut part_in_this_place_already_exist = false;
                        let x = rand::thread_rng().gen_range(2..=18);
                        let y = rand::thread_rng().gen_range(2..=18);

                        for part in &mut snake.body_parts {
                            if part.x == x as f32 * resources::BLOCKSIZE && part.y == y as f32 * resources::BLOCKSIZE {
                                part_in_this_place_already_exist = true;
                                break;
                            }
                        }

                        if snake.x == x as f32 && snake.y == y as f32 {
                            part_in_this_place_already_exist = true;
                        }

                        if !part_in_this_place_already_exist {
                            apples.push(
                                Apple::new(x as f32 * resources::BLOCKSIZE, y as f32 * resources::BLOCKSIZE, &resources).await,
                            );
                            item_placed_successfully = true;
                        }
                    }
                }

                snake.draw();
            },
            GameState::LevelFail => {
                draw_map(&resources);
                snake.draw();

                show_text(resources.font, "Game over", "Press 'space' to start new game...");

                if is_key_pressed(KeyCode::Space) {
                    game_state = GameState::InitLevel;
                }
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