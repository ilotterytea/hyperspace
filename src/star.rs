use std::{f32::consts::PI, ops::Range};

use ::rand::{thread_rng, Rng};
use macroquad::prelude::*;

pub const STAR_AMOUNT: i32 = 1000;
const STAR_POS_Z: Range<i32> = 10..20;
const SPACE_SIZE: f32 = 32.0;
const STAR_VELOCITY_RANGE: Range<i32> = 1..5;

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
            color: WHITE,
        }
    }

    pub fn update(&mut self, screen_center_x: f32, screen_center_y: f32) {
        let x = self.position.x / self.position.z + screen_center_x;
        let y = self.position.y / self.position.z + screen_center_y;

        self.render_position.x = x;
        self.render_position.y = y;

        let size = (STAR_POS_Z.end as f32 - self.position.z) / (0.2 * self.position.z);

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

    Vec3::new(x, y, rand.gen_range(STAR_POS_Z) as f32)
}
