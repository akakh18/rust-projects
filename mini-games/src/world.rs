use macroquad::prelude::*;
use crate::snake::Snake;
use crate::direction::Direction;
use crate::config::{Config, CONFIG};
use crate::utils::random;

pub struct Food {
    pub col: i32,
    pub row: i32,
    pub color: Color,
}

impl Food {
    pub fn new(config: &Config, snake: &Snake) -> Self {
        loop {
            let col = random(0, config.get_num_cols());
            let row = random(0, config.get_num_rows());

            if !snake.parts.iter().any(|p| p.col == col && p.row == row) {
                return Food {
                    col,
                    row,
                    color: config.color_food,
                };
            }
        }
    }

    pub fn draw(&self, margin_x: f32, margin_y: f32, cell_w: f32, cell_h: f32) {
        let x = margin_x + self.col as f32 * cell_w;
        let y = margin_y + self.row as f32 * cell_h;
        draw_rectangle(x, y, cell_w, cell_h, self.color);
    }
}



pub struct World {
    pub snake: Snake,
    pub direction: Direction,
    pub game_over: bool,
    pub food: Food,
}

impl World {
    pub fn new(snake: Snake, direction: Direction) -> Self {
        let food = Food::new(&CONFIG, &snake);
        World { snake, direction, game_over: false, food }
    }

    pub fn update_direction(&mut self) {
        let dx = self.direction.dx;
        let dy = self.direction.dy;

        if is_key_down(KeyCode::Right) && dx != -1 {
            self.direction.dx = 1;
            self.direction.dy = 0;
        } else if is_key_down(KeyCode::Left) && dx != 1 {
            self.direction.dx = -1;
            self.direction.dy = 0;
        } else if is_key_down(KeyCode::Down) && dy != -1 {
            self.direction.dx = 0;
            self.direction.dy = 1;
        } else if is_key_down(KeyCode::Up) && dy != 1 {
            self.direction.dx = 0;
            self.direction.dy = -1;
        }
    }

    pub fn update(&mut self) {
        if self.game_over { return; }

        self.update_direction();
        self.snake.update_position(&self.direction);

        if self.snake.collides() {
            self.game_over = true;
        }

        let head = &self.snake.parts[0];
        if head.col == self.food.col && head.row == self.food.row {
            self.snake.grow();
            self.food = Food::new(&CONFIG, &self.snake);
        }
    }


    pub fn draw(&mut self, frame_count: u64) {
        let margin_x = screen_width() * 0.02;
        let margin_y = screen_height() * 0.02;
        let cols = CONFIG.get_num_cols();
        let rows = CONFIG.get_num_rows();
        let cell_w = (screen_width() - 2.0 * margin_x) / cols as f32;
        let cell_h = (screen_height() - 2.0 * margin_y) / rows as f32;

        draw_rectangle(0.0, 0.0, screen_width(), screen_height(), DARKGRAY);
        draw_rectangle(
            margin_x,
            margin_y,
            screen_width() - 2.0 * margin_x,
            screen_height() - 2.0 * margin_y,
            BLACK,
        );

        for i in 1..cols {
            let x = margin_x + i as f32 * cell_w;
            draw_line(x, margin_y, x, screen_height() - margin_y, 0.8, WHITE);
        }

        for j in 1..rows {
            let y = margin_y + j as f32 * cell_h;
            draw_line(margin_x, y, screen_width() - margin_x, y, 0.8, WHITE);
        }

        self.food.draw(margin_x, margin_y, cell_w, cell_h);
        self.snake.draw(margin_x, margin_y, cell_w, cell_h);


        if self.game_over {
            let text = "GAME OVER!";
            let font_size = screen_width() * 0.05;
            let text_size = measure_text(text, None, font_size as u16, 1.0);
            let color = if frame_count / 30 % 2 == 0 { RED } else { WHITE };

            draw_text(
                text,
                screen_width() / 2.0 - text_size.width / 2.0,
                screen_height() / 2.0,
                font_size,
                color,
            );

            self.restart();
        }
    }

    pub fn restart(&mut self) {
        let btn_w = screen_width() * 0.3;
        let btn_h = screen_height() * 0.08;
        let btn_x = screen_width() / 2.0 - btn_w / 2.0;
        let btn_y = screen_height() / 2.0 + screen_height() * 0.05;

        draw_rectangle(btn_x, btn_y, btn_w, btn_h, DARKGRAY);
        draw_text(
            "RESTART",
            btn_x + btn_w * 0.2,
            btn_y + btn_h * 0.65,
            btn_h * 0.4,
            WHITE,
        );

        if is_mouse_button_pressed(MouseButton::Left) {
            let (mx, my) = mouse_position();
            if mx >= btn_x && mx <= btn_x + btn_w && my >= btn_y && my <= btn_y + btn_h {
                self.init_world();
            }
        }
    }

    pub fn init_world(&mut self) {
        self.snake = Snake::new();
        self.direction = Direction::new();
        self.game_over = false;
    }
}
