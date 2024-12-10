use crate::star::*;
use macroquad::prelude::*;

mod star;

#[macroquad::main("hyperspace")]
async fn main() {
    let mut stars: Vec<Star> = Vec::new();
    for _ in 0..STAR_AMOUNT {
        stars.push(Star::new());
    }

    loop {
        clear_background(WHITE);

        for star in &mut stars {
            star.position.z -= star.velocity;

            if star.position.z < 1.0 {
                star.position = generate_3d_position();
            }

            star.update();

            draw_rectangle(
                star.render_position.x,
                star.render_position.y,
                star.size.x,
                star.size.y,
                star.color,
            );
        }

        next_frame().await
    }
}
