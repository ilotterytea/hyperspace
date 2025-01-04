use crate::star::*;
use clap::Parser;
use macroquad::prelude::*;

mod star;

#[derive(Parser, Debug)]
struct Args {
    #[arg(short, long)]
    pub texture_path: Option<String>,
}

#[macroquad::main("hyperspace (demo)")]
async fn main() {
    let args = Args::parse();
    let mut mouse_control = false;

    // loading a texture
    let mut texture: Option<Texture2D> = None;

    if args.texture_path.is_some() {
        match load_texture(args.texture_path.unwrap().as_str()).await {
            Ok(x) => texture = Some(x),
            Err(_) => warn!("Failed to load a star texture"),
        }
    }

    let mut stars: Vec<Star> = Vec::new();
    for _ in 0..STAR_AMOUNT {
        stars.push(Star::new(&texture));
    }

    loop {
        clear_background(BLACK);

        stars.sort_by_key(|x| x.position.z as i32);
        stars.sort_by(|a, b| b.position.z.partial_cmp(&a.position.z).unwrap());

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

            if star.position.z <= 5.0 {
                let x = star.render_position.x.powf(star.position.z / 10.0).abs();
                if star.render_position.x < screen_center_x {
                    star.position.x -= x;
                }
                if star.render_position.x > screen_center_x {
                    star.position.x += x;
                }
            }

            star.update(screen_center_x, screen_center_y);
            star.color.a = 255.0
                / if star.position.z - 5.0 < 1.0 {
                    1.0
                } else {
                    star.position.z - 5.0
                }
                / 255.0;

            if star.texture.is_some() {
                draw_texture_ex(
                    star.texture.as_ref().unwrap(),
                    star.render_position.x,
                    star.render_position.y,
                    star.color,
                    DrawTextureParams {
                        dest_size: Some(star.size),
                        ..DrawTextureParams::default()
                    },
                );
            } else {
                draw_rectangle(
                    star.render_position.x,
                    star.render_position.y,
                    star.size.x,
                    star.size.y,
                    star.color,
                );
            }
        }

        if is_mouse_button_pressed(MouseButton::Left) {
            mouse_control = !mouse_control;
            show_mouse(!mouse_control);
        }

        next_frame().await
    }
}
