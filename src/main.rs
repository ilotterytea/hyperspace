use macroquad::prelude::*;

#[macroquad::main("hyperspace")]
async fn main() {
    loop {
        clear_background(WHITE);

        draw_text("hi world!", 20.0, 20.0, 30.0, DARKGRAY);

        next_frame().await
    }
}
