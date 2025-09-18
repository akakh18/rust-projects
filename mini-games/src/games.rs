use crate::game::Game; // <- import the unified trait
use std::pin::Pin;
use std::future::Future;
use macroquad::prelude::*;
use crate::direction::Direction;
use crate::snake::Snake;
use crate::world::World;

pub struct SnakeGame {}

impl Game for SnakeGame {
    fn start(&self) -> Pin<Box<dyn Future<Output = ()> + 'static>> {
        Box::pin(async move {
            let snake = Snake::new();
            let direction = Direction::new();
            let mut world = World::new(snake, direction);

            let mut frame_count: u64 = 0;
            let mut elapsed_time: f32 = 0.0;
            let move_interval: f32 = 0.12;

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
        })
    }
}
