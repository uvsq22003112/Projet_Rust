use macroquad::prelude::*;

pub struct Missile {
    position: Vec2,
    speed: f32,
    direction: Vec2,
}

impl Missile {
    pub const MISSILE_SPEED: f32 = 5.0;

    pub fn new(position: Vec2, direction: Vec2) -> Self {
        Self {
            position,
            speed: Self::MISSILE_SPEED,
            direction: direction.normalize(),
        }
    }

    pub fn update_position(&mut self) {
        self.position += self.direction * self.speed;
    }

    pub fn hors_ecran(&self) -> bool {
        self.position.x < 0.0 || self.position.x > screen_width() ||
        self.position.y < 0.0 || self.position.y > screen_height()
    }

    pub fn get_position(&self) -> Vec2 {
        self.position
    }
}

