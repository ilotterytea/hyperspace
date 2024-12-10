use std::{f32::consts::PI, ops::Range};

use ::rand::{thread_rng, Rng};
use macroquad::prelude::*;

pub const STAR_AMOUNT: i32 = 1000;
pub const STAR_START_POS_Z: f32 = 20.0;
pub const SPACE_SIZE: f32 = 32.0;
pub const STAR_VELOCITY_RANGE: Range<i32> = 1..25;

pub struct Star {
    pub position: Vec3,
    pub render_position: Vec2,
    pub size: Vec2,
    pub velocity: f32,
    pub color: Color,
}

impl Star {
    pub fn new() -> Self {
        let mut rand = thread_rng();
        Self {
            position: generate_3d_position(),
            render_position: Vec2::new(0.0, 0.0),
            size: Vec2::new(1.0, 1.0),
            velocity: rand.gen_range(STAR_VELOCITY_RANGE) as f32 / 100.0,
            color: BLACK,
        }
    }

    pub fn update(&mut self) {
        let screen_center_x = screen_width() / 2.0;
        let screen_center_y = screen_height() / 2.0;

        let x = self.position.x / self.position.z + screen_center_x;
        let y = self.position.y / self.position.z + screen_center_y;

        self.render_position.x = x;
        self.render_position.y = y;

        let size = (STAR_START_POS_Z - self.position.z) / (0.2 * self.position.z);

        self.size.x = size;
        self.size.y = size;
    }
}

pub fn generate_3d_position() -> Vec3 {
    let mut rand = thread_rng();
    let angle = rand.gen::<f32>() * 2.0 * PI;
    let radius =
        (screen_height() / SPACE_SIZE) + (rand.gen::<f32>() * screen_height()) * SPACE_SIZE;

    let x = radius * angle.sin();
    let y = radius * angle.cos();

    Vec3::new(x, y, STAR_START_POS_Z)
}
