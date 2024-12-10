use macroquad::prelude::*;

pub const STAR_AMOUNT: i32 = 1000;

pub struct Star {
    pub position: Vec3,
    pub size: Vec2,
    pub velocity: f32,
    pub color: Color,
}

impl Star {}
