use crate::star::*;
use macroquad::prelude::*;

mod star;

#[macroquad::main("hyperspace")]
async fn main() {
    let mut mouse_control = false;

    let mut stars: Vec<Star> = Vec::new();
    for _ in 0..STAR_AMOUNT {
        stars.push(Star::new());
    }

    loop {
        clear_background(BLACK);

        stars.sort_by_key(|x| x.position.z as i32);

        for star in &mut stars {
            star.position.z -= star.velocity;

            if star.position.z < 1.0 {
                star.color.a = 0.0;
                star.position = generate_3d_position();
            }

            let (screen_center_x, screen_center_y) = if mouse_control {
                mouse_position()
            } else {
                (screen_width() / 2.0, screen_height() / 2.0)
            };

            star.update(screen_center_x, screen_center_y);
            star.color.a = 255.0
                / if star.position.z - 5.0 < 1.0 {
                    1.0
                } else {
                    star.position.z - 5.0
                }
                / 255.0;

            draw_rectangle(
                star.render_position.x,
                star.render_position.y,
                star.size.x,
                star.size.y,
                star.color,
            );
        }

        if is_mouse_button_pressed(MouseButton::Left) {
            mouse_control = !mouse_control;
            show_mouse(!mouse_control);
        }

        next_frame().await
    }
}
