use macroquad::prelude::*;

mod direction;
mod snake;
mod world;
mod config;
mod utils;

use direction::Direction;
use world::World;
use crate::snake::Snake;

#[macroquad::main("Snake Game")]
async fn main() {
    let snake = Snake::new();
    let direction = Direction::new();
    let mut world = World::new(snake, direction);

    let mut frame_count: u64 = 0;
    let mut elapsed_time: f32 = 0.0;
    let move_interval: f32 = 0.2;

    loop {
        clear_background(WHITE);

        elapsed_time += get_frame_time();
        world.update_direction();

        if elapsed_time >= move_interval {
            world.update();
            elapsed_time = 0.0;
        }

        world.draw(frame_count);

        frame_count += 1;

        next_frame().await;
    }
}
