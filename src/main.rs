use crate::star::*;
use macroquad::prelude::*;

mod star;

#[macroquad::main("hyperspace")]
async fn main() {
    let mut stars: Vec<Star> = Vec::new();
    for _ in 0..STAR_AMOUNT {
        stars.push(Star {
            position: Vec3::new(0.0, 0.0, 0.0),
            size: Vec2::new(20.0, 20.0),
            velocity: 1.0,
            color: BLACK,
        });
    }

    loop {
        clear_background(WHITE);

        for star in &stars {
            draw_rectangle(
                star.position.x,
                star.position.y,
                star.size.x,
                star.size.y,
                star.color,
            );
        }

        next_frame().await
    }
}
