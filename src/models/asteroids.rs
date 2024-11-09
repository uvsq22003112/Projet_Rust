use std::f32::consts::PI; // Utiliser PI en tant que f32
use macroquad::prelude::*;
use ::rand::{thread_rng, Rng}; // Préciser `::rand` pour éviter les ambiguïtés

// Enum pour la taille d'un astéroïde : grand, moyen, petit.
pub enum AsteroidSize {
    Large,
    Medium,
    Small,
}

// Structure de l'astéroïde avec position, vitesse, et taille.
pub struct Asteroid {
    pub position: Vec2,    // Position actuelle de l'astéroïde
    pub speed: Vec2,       // Vitesse de déplacement
    pub size: AsteroidSize, // Taille de l'astéroïde
}

impl Asteroid {
    pub const ASTEROID_LARGE_SIZE: f32 = 60.0;
    pub const ASTEROID_MEDIUM_SIZE: f32 = 30.0;
    pub const ASTEROID_SMALL_SIZE: f32 = 15.0;

    // Constructeur pour créer un nouvel astéroïde avec une taille spécifiée
    pub fn new(size: AsteroidSize) -> Self {
        Self {
            position: Self::new_alea_position(&size),
            speed: Self::new_alea_speed(&size),
            size,
        }
    }

    // Retourne la taille en pixels selon la catégorie de l'astéroïde
    pub fn get_size_asteroids(&self) -> f32 {
        match self.size {
            AsteroidSize::Large => Asteroid::ASTEROID_LARGE_SIZE,
            AsteroidSize::Medium => Asteroid::ASTEROID_MEDIUM_SIZE,
            AsteroidSize::Small => Asteroid::ASTEROID_SMALL_SIZE,
        }
    }

    // Génère une position de départ aléatoire sur un côté de l'écran
    fn new_alea_position(size: &AsteroidSize) -> Vec2 {
        let mut rng = thread_rng();
    
        // Calcule une distance aléatoire par rapport au bord
        let nearpos = match size {
            AsteroidSize::Large => rng.gen_range(Self::ASTEROID_LARGE_SIZE / 2.0..=Self::ASTEROID_LARGE_SIZE),
            AsteroidSize::Medium => rng.gen_range(Self::ASTEROID_MEDIUM_SIZE / 2.0..=Self::ASTEROID_MEDIUM_SIZE),
            AsteroidSize::Small => rng.gen_range(Self::ASTEROID_SMALL_SIZE / 2.0..=Self::ASTEROID_SMALL_SIZE),
        };
    
        // Choisit un côté aléatoire : 1 = haut, 2 = droite, 3 = bas, 4 = gauche
        let nearside = rng.gen_range(1..=4);
    
        // Position x selon le côté choisi
        let xpos: f32 = match nearside {
            2 => screen_width() - nearpos,          // Droite
            4 => nearpos,                           // Gauche
            _ => rng.gen_range(0.0..=screen_width()), // Haut ou bas : position x aléatoire
        };
    
        // Position y selon le côté choisi
        let ypos: f32 = match nearside {
            1 => nearpos,                           // Haut
            3 => screen_height() - nearpos,         // Bas
            _ => rng.gen_range(0.0..=screen_height()), // Droite ou gauche : position y aléatoire
        };
    
        vec2(xpos, ypos)
    }
    
    // Génère une vitesse aléatoire selon la taille et une direction aléatoire
    fn new_alea_speed(size: &AsteroidSize) -> Vec2 {
        let mut rng = thread_rng();
    
        let angle: f32 = rng.gen_range(0.0..=2.0 * PI); // Angle de direction aléatoire
    
        let speed_intensity = match size {                // Vitesse en fonction de la taille
            AsteroidSize::Large => rng.gen_range(0.5..=1.0),
            AsteroidSize::Medium => rng.gen_range(1.0..=2.0),
            AsteroidSize::Small => rng.gen_range(2.0..=3.0),
        };
    
        Vec2::from_angle(angle) * speed_intensity
    }

    // Retourne la position actuelle de l'astéroïde
    pub fn get_position(&self) -> Vec2 {
        self.position
    }

    // Met à jour la position en fonction de la vitesse et applique le wrapping
    pub fn move_object(&mut self) -> Vec2 {
        self.position += self.speed;
        self.position = Self::bound_pos(self.position); // Applique les limites d'écran
        self.position
    }

    // Applique le wrapping sur les axes x et y
    fn bound_pos(mut pos: Vec2) -> Vec2 {
        pos.x = Self::bound_to(pos.x, screen_width());
        pos.y = Self::bound_to(pos.y, screen_height());
        pos
    }

    // Ajuste la coordonnée pour qu'elle reste dans la plage [0, max] (wrapping)
    fn bound_to(coord: f32, max: f32) -> f32 {
        if coord < 0.0 {
            max + coord
        } else if coord > max {
            coord - max
        } else {
            coord
        }
    }
}

