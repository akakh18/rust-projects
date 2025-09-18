use std::future::Future;
use crate::game::Game;
use macroquad::prelude::*;
use macroquad::color::{Color, WHITE};
use macroquad::input::{is_mouse_button_pressed, MouseButton};
use std::pin::Pin;

pub struct GameButton<'a> {
    pub btn_w: f32,
    pub btn_h: f32,
    pub btn_x: f32,
    pub btn_y: f32,
    pub color: Color,
    pub text: String,
    pub hover_color: Color,
    pub game: Option<&'a dyn Game>,
}

impl<'a> GameButton<'a> {
    pub fn new(
        btn_w: f32,
        btn_h: f32,
        btn_x: f32,
        btn_y: f32,
        color: Color,
        hover_color: Color,
        text: String,
        game: Option<&'a dyn Game>,
    ) -> Self {
        GameButton { btn_w, btn_h, btn_x, btn_y, color, hover_color, text, game }
    }

    fn is_hovered(&self) -> bool {
        let (mx, my) = mouse_position();
        mx >= self.btn_x && mx <= self.btn_x + self.btn_w && my >= self.btn_y && my <= self.btn_y + self.btn_h
    }

    pub fn draw(&self) -> Option<Pin<Box<dyn Future<Output = ()> + 'static>>> {
        let base_color = self.color;
        let hover_color = self.hover_color;
        let current_color = if self.is_hovered() { hover_color } else { base_color };

        let shadow_offset = 4.0;
        draw_rectangle(
            self.btn_x + shadow_offset,
            self.btn_y + shadow_offset,
            self.btn_w,
            self.btn_h,
            Color::new(0.0, 0.0, 0.0, 0.3),
        );

        draw_rectangle(self.btn_x, self.btn_y, self.btn_w, self.btn_h, current_color);
        draw_rectangle_lines(self.btn_x, self.btn_y, self.btn_w, self.btn_h, 3.0, WHITE);

        if self.is_hovered() {
            draw_rectangle_lines(self.btn_x - 2.0, self.btn_y - 2.0, self.btn_w + 4.0, self.btn_h + 4.0, 2.0, WHITE);
        }

        let text_size = self.btn_h * 0.4;
        let text_dim = measure_text(&self.text, None, text_size as u16, 1.0);
        let text_x = self.btn_x + (self.btn_w - text_dim.width) / 2.0;
        let text_y = self.btn_y + (self.btn_h + text_dim.height) / 2.0 - 2.0; // vertical center

        draw_text(&self.text, text_x, text_y, text_size, WHITE);

        if is_mouse_button_pressed(MouseButton::Left) && self.is_hovered() {
            return self.game.map(|g| g.start());
        }

        None
    }
}

pub struct GameMenu<'a> {
    pub buttons: Vec<GameButton<'a>>,
    pub background: Texture2D,
}

impl<'a> GameMenu<'a> {
    pub async fn new(buttons: Vec<GameButton<'a>>, bg_path: &str) -> Self {
        let background = load_texture(bg_path).await.unwrap();
        background.set_filter(FilterMode::Linear);
        GameMenu { buttons, background }
    }

    pub fn draw(&self) -> Option<Pin<Box<dyn Future<Output = ()> + 'static>>> {
        draw_texture(&self.background, 0.0, 0.0, WHITE);

        for button in &self.buttons {
            if let Some(fut) = button.draw() {
                return Some(fut);
            }
        }
        None
    }
}
