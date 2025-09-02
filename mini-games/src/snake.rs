use macroquad::prelude::*;
use std::time::SystemTime;
use crate::config::{CONFIG};

pub struct SnakePart {
    pub col: i32,
    pub row: i32,
    pub color: Color,
}

impl SnakePart {
    pub fn new(col: i32, row: i32, color: Color) -> SnakePart {
        SnakePart { col, row, color }
    }

    pub fn draw(&self, margin_x: f32, margin_y: f32, cell_w: f32, cell_h: f32) {
        let x = margin_x + self.col as f32 * cell_w;
        let y = margin_y + self.row as f32 * cell_h;
        draw_rectangle(x, y, cell_w, cell_h, self.color);
    }

    pub fn collides_with_wall(&self, cols: i32, rows: i32) -> bool {
        self.col < 0 || self.row < 0 || self.col >= cols || self.row >= rows
    }
}

pub struct Snake {
    pub parts: Vec<SnakePart>,
    pub last_x: i32,
    pub last_y: i32
}


fn random(low: i32, high: i32) -> i32 {
    let timestamp = (SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_millis() as i32)
        .abs();
    low + (timestamp % (high - low))
}

impl Snake {
    pub fn new() -> Self {
        let head_col = crate::utils::random(CONFIG.get_snake_init_size(), CONFIG.get_num_cols());
        let head_row = crate::utils::random(0, CONFIG.get_num_rows());

        let mut snake_parts: Vec<SnakePart> = vec![
            SnakePart::new(head_col, head_row, CONFIG.color_snake_head)
        ];
        let mut last_x = 0;
        let mut last_y = 0;
        for i in 1..CONFIG.get_snake_init_size() {
            let current_snake_part = SnakePart::new(head_col - i, head_row, CONFIG.color_snake_body);
            last_y = current_snake_part.row;
            last_x = current_snake_part.col;
            snake_parts.push(current_snake_part);
        }

        Snake { parts: snake_parts, last_y, last_x }
    }


    pub fn update_position(&mut self, direction: &crate::direction::Direction) {
        if self.parts.is_empty() {
            return;
        }

        self.last_x = self.parts.last().unwrap().col;
        self.last_y = self.parts.last().unwrap().row;


        let mut prev_col = self.parts[0].col;
        let mut prev_row = self.parts[0].row;

        let head = &mut self.parts[0];
        head.col += direction.dx;
        head.row += direction.dy;

        if direction.dx != 0 || direction.dy != 0 {
            for i in 1..self.parts.len() {
                let part = &mut self.parts[i];
                let temp_col = part.col;
                let temp_row = part.row;
                part.col = prev_col;
                part.row = prev_row;
                prev_col = temp_col;
                prev_row = temp_row;
            }
        }
    }

    pub fn grow(&mut self) {
        self.parts.push(SnakePart::new(self.last_x, self.last_y, SKYBLUE));
    }

    pub fn draw(&self, margin_x: f32, margin_y: f32, cell_w: f32, cell_h: f32) {
        for part in &self.parts {
            part.draw(margin_x, margin_y, cell_w, cell_h);
        }
    }

    pub fn collides(&self) -> bool {
        if let Some((head, body)) = self.parts.split_first() {
            if head.collides_with_wall(CONFIG.get_num_cols(), CONFIG.get_num_rows()) {
                return true;
            }

            for part in body {
                if part.row == head.row && part.col == head.col {
                    return true;
                }
            }
        }
        false
    }
}
