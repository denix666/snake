#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use macroquad::prelude::*;

mod resources;
use resources::Resources;

mod snake;
use snake::Snake;

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

    loop {
        clear_background(BLACK);
        
        match game_state {
            GameState::Intro=>{
                game_state = GameState::InitLevel;
            },
            GameState::InitLevel => {
                game_state = GameState::Game;
            },
            GameState::Game => {
                draw_map(&resources);
                
                snake.update();
                snake.draw();
            },
            GameState::LevelFail => {

            },
            GameState::Paused => {

            }, 
        }

        next_frame().await
    }
}