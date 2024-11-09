use macroquad::prelude::*;
use crate::models::missiles::Missile;

pub struct Spaceship {
    position: Vec2,
    orientation: Vec2,
}

impl Spaceship {
    pub const SHIP_RADIUS: f32 = 10.0;
    pub const SPEED: f32 = 2.0;

    pub fn new() -> Self {
        Self {
            position: vec2(screen_width() / 2.0, screen_height() / 2.0),
            orientation: vec2(0.0, -1.0), // Orientation initiale vers le haut
        }
    }
    
    pub fn get_position(&self) -> Vec2 {
        self.position
    }

    pub fn get_orientation(&self) -> Vec2 {
        self.orientation
    }

    // Définit une nouvelle orientation pour le vaisseau
    pub fn set_orientation(&mut self, new_orientation: Vec2) {
        self.orientation = new_orientation.normalize();
    }

    // Applique une poussée dans la direction actuelle de l'orientation
    pub fn apply_thrust(&mut self) {
        self.position += self.orientation * Self::SPEED;
    }

    // Wrapping de l'écran pour les coordonnées x et y
    pub fn update_position(&mut self) {
        if self.position.x < 0.0 {
            self.position.x = screen_width();
        } else if self.position.x > screen_width() {
            self.position.x = 0.0;
        }

        if self.position.y < 0.0 {
            self.position.y = screen_height();
        } else if self.position.y > screen_height() {
            self.position.y = 0.0;
        }
    }

    // Méthode pour tirer un missile dans la direction actuelle du vaisseau
    pub fn tire(&self) -> Missile {
        Missile::new(self.position, self.orientation)
    }
}


