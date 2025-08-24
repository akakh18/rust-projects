use macroquad::prelude::*;

#[macroquad::main("Simple Game")]
async fn main() {
    let mut x = 100.0;
    let mut y = 100.0;
    loop {
        clear_background(BLACK);

        if is_key_down(KeyCode::Right) {
            x += 2.0;
        }
        if is_key_down(KeyCode::Left) {
            x -= 2.0;
        }
        if is_key_down(KeyCode::Down) {
            y += 2.0;
        }
        if is_key_down(KeyCode::Up) {
            y -= 2.0;
        }

        draw_rectangle(x, y, 50.0, 50.0, RED);

        next_frame().await
    }
}
