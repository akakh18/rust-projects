use std::pin::Pin;
use std::future::Future;

mod menu;
mod games;
mod direction;
mod snake;
mod world;
mod config;
mod utils;
mod game;

use macroquad::prelude::*;
use crate::menu::{GameButton, GameMenu};
use crate::games::SnakeGame;
use crate::game::Game;

#[macroquad::main("Snake Game")]
async fn main() {
    let snake_game = SnakeGame {};

    let buttons = vec![
        GameButton::new(
            screen_width() * 0.3,
            screen_height() * 0.08,
            screen_width() / 2.0 - screen_width() * 0.3 / 2.0,
            screen_height() / 2.0 - screen_height() * 0.05,
            DARKBLUE,
            BLUE,
            "SNAKE".to_string(),
            Some(&snake_game),
        )
    ];

    let menu = GameMenu::new(buttons, "assets/background.png").await;

    let mut running_game: Option<Pin<Box<dyn Future<Output = ()>>>> = None;

    loop {
        clear_background(WHITE);

        if let Some(game_future) = &mut running_game {
            game_future.as_mut().await;
            running_game = None;
        } else {
            if let Some(game_future) = menu.draw() {
                running_game = Some(game_future);
            }
        }

        next_frame().await;
    }
}
