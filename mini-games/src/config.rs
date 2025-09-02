use macroquad::prelude::*;
use once_cell::sync::Lazy;

pub struct Config {
    num_rows: i32,
    num_cols: i32,
    snake_init_size: i32,
    pub color_bg: Color,
    pub color_grid: Color,
    pub color_snake_head: Color,
    pub color_snake_body: Color,
    pub color_food: Color,
}

impl Config {
    pub fn default() -> Self {
        Self {
            num_rows: 18,
            num_cols: 18,
            snake_init_size: 8,
            color_bg: DARKGRAY,
            color_grid: WHITE,
            color_snake_head: DARKBLUE,
            color_snake_body: SKYBLUE,
            color_food: RED,
        }
    }

    pub fn get_num_cols(&self) -> i32 {
        self.num_cols
    }

    pub fn get_num_rows(&self) -> i32 {
        self.num_rows
    }

    pub fn get_snake_init_size(&self) -> i32 {
        self.snake_init_size
    }
}

pub static CONFIG: Lazy<Config> = Lazy::new(|| Config::default());