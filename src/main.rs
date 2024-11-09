use macroquad::prelude::*;
use ::rand::{thread_rng, Rng};

mod models;

use models::asteroids::{Asteroid, AsteroidSize};
use models::ship::Spaceship;
use models::missiles::Missile;



fn draw(asteroids: &[Asteroid], spaceship: &Spaceship, missiles: &[Missile]) {
    draw_background();

    for asteroid in asteroids {
        draw_asteroid(asteroid);
    }

    draw_spaceship(spaceship);

    for missile in missiles {
        draw_missile(missile);
    }
}

fn draw_background() {
    clear_background(BLACK);
}

fn draw_asteroid(asteroid: &Asteroid) {
    let radius = asteroid.get_size_asteroids();
    let position = asteroid.get_position();
    draw_circle_lines(position.x, position.y, radius, 1.0, YELLOW);
}

fn draw_spaceship(spaceship: &Spaceship) {
    let position = spaceship.get_position();
    draw_circle(position.x, position.y, Spaceship::SHIP_RADIUS, BLUE);

    let orientation_point = position + spaceship.get_orientation() * Spaceship::SHIP_RADIUS * 1.5;
    draw_circle(orientation_point.x, orientation_point.y, 3.0, RED);
}

fn draw_missile(missile: &Missile) {
    let position = missile.get_position();
    draw_circle(position.x, position.y, 2.0, WHITE);
}

fn handle_input() -> bool {
    is_key_down(KeyCode::Escape)
}

fn handle_input_spaceship(spaceship: &mut Spaceship, missiles: &mut Vec<Missile>) {
    // Vérifie si les touches de direction sont enfoncées et oriente le vaisseau en conséquence

    if is_key_down(KeyCode::Up) {
        // Oriente vers le haut et applique la poussée
        spaceship.set_orientation(vec2(0.0, -1.0));
        spaceship.apply_thrust();
    }
    if is_key_down(KeyCode::Down) {
        // Oriente vers le bas et applique la poussée
        spaceship.set_orientation(vec2(0.0, 1.0));
        spaceship.apply_thrust();
    }
    if is_key_down(KeyCode::Left) {
        // Oriente vers la gauche et applique la poussée
        spaceship.set_orientation(vec2(-1.0, 0.0));
        spaceship.apply_thrust();
    }
    if is_key_down(KeyCode::Right) {
        // Oriente vers la droite et applique la poussée
        spaceship.set_orientation(vec2(1.0, 0.0));
        spaceship.apply_thrust();
    }

    // Vérifie si la touche 'Space' est pressée pour tirer un missile
    if is_key_pressed(KeyCode::Space) {
        missiles.push(spaceship.tire());
    }
}



fn update_model(asteroids: &mut Vec<Asteroid>, spaceship: &mut Spaceship, missiles: &mut Vec<Missile>) {
    for asteroid in asteroids.iter_mut() {
        asteroid.move_object();
    }

    spaceship.update_position();

    for missile in missiles.iter_mut() {
        missile.update_position();
    }

    missiles.retain(|missile| !missile.hors_ecran());
}

#[macroquad::main("Spaceship Game")]
async fn main() {
    let mut asteroids = vec![];
    let mut rng = thread_rng();
    let mut spaceship = Spaceship::new();
    let mut missiles = vec![];

    for _ in 0..10 {
        let size = match rng.gen_range(0..3) {
            2 => AsteroidSize::Large,
            1 => AsteroidSize::Medium,
            _ => AsteroidSize::Small,
        };
        asteroids.push(Asteroid::new(size));
    }

    loop {
        draw(&asteroids, &spaceship, &missiles);

        if handle_input() {
            break;
        }

        handle_input_spaceship(&mut spaceship, &mut missiles);

        update_model(&mut asteroids, &mut spaceship, &mut missiles);

        next_frame().await;
    }
}
