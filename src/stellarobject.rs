//! Module définissant les comportements communs des objets du jeu.
//!
//! Ce module contient le trait `StellarObject` qui définit les comportements
//! partagés entre les différents objets du jeu (astéroïdes, vaisseau, projectiles).

use crate::{asteroid::Asteroid, bullet::Bullet, spaceship::Spaceship};

/// Trait définissant les comportements communs aux objets du jeu
///
/// # Méthodes requises
/// * `move_object` - Déplace l'objet selon son vecteur de vitesse
/// * `collided` - Gère le comportement lors d'une collision
pub trait StellarObject {
    fn move_object(&mut self);
    fn collided(&mut self, other: &mut Asteroid) -> bool;
}

impl StellarObject for Asteroid {
    fn move_object(&mut self) {
        self.position += self.get_speed();
    }

    fn collided(&mut self, other: &mut Asteroid) -> bool {
        let delta = other.get_position() - self.get_position();
        let distance = delta.length();
        let min_distance = self.get_width() + other.get_width();
        if distance < min_distance {
            let overlap = min_distance - distance;
            let correction = delta.normalize() * overlap / 2.0;

            self.handle_collision(-correction);
            other.handle_collision(correction);

            true
        } else {
            false
        }
    }
}

impl StellarObject for Spaceship {
    fn move_object(&mut self) {
        self.position += self.get_speed();
    }

    fn collided(&mut self, other: &mut Asteroid) -> bool {
        let delta = other.get_position() - self.get_position();
        let distance = delta.length();
        let min_distance = self.get_height() + other.get_width();
        if distance < min_distance {
            let overlap = min_distance - distance;
            let correction = delta.normalize() * overlap / 2.0;

            self.handle_collision(-correction);
            other.handle_collision(correction);

            true
        } else {
            false
        }
    }
}

impl StellarObject for Bullet {
    fn move_object(&mut self) {
        self.position += self.get_speed();
    }

    fn collided(&mut self, other: &mut Asteroid) -> bool {
        let delta = other.get_position() - self.get_position();
        let distance = delta.length();
        let min_distance = self.get_size() + other.get_width();
        if distance < min_distance {
            self.handle_collision();
            other.set_touched();

            true
        } else {
            false
        }
    }
}
