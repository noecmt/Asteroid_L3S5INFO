//! Module gérant le vaisseau spatial du jeu Asteroids.
//!
//! Ce module contient la structure `Spaceship` et toutes ses méthodes associées
//! pour gérer le déplacement, l'orientation et les collisions du vaisseau.

use macroquad::prelude::*;

/// Représente le vaisseau spatial contrôlé par le joueur.
///
/// # Champs
/// * `position` - Position actuelle du vaisseau (Vec2)
/// * `speed` - Vecteur vitesse du vaisseau
/// * `orientation` - Angle d'orientation en radians
/// * `shield` - Points de bouclier restants
/// * `invincibility_timer` - Temps d'invincibilité restant après une collision
pub struct Spaceship {
    pub position: Vec2,
    speed: Vec2,
    orientation: f32,
    shield: u8,
    invincibility_timer: f32,
}

impl Spaceship {
    /// Constantes définissant les caractéristiques du vaisseau
    pub const SPACESHIP_HEIGHT: f32 = 25.0;
    pub const SPACESHIP_MAX_SPEED: f32 = 3.;
    pub const SPACESHIP_MIN_SPEED: f32 = 0.5;

    /// Crée une nouvelle instance de Spaceship au centre de l'écran
    pub fn new() -> Self {
        Self {
            position: Vec2::new(screen_width() / 2., screen_height() / 2.),
            speed: Vec2::new(0.0, -0.7),
            orientation: 0.0,
            shield: 10,
            invincibility_timer: 0.0,
        }
    }

    pub fn get_height(&self) -> f32 {
        Self::SPACESHIP_HEIGHT
    }

    pub fn get_position(&self) -> Vec2 {
        self.position
    }

    pub fn get_speed(&self) -> Vec2 {
        self.speed
    }

    pub fn get_orientation(&self) -> f32 {
        self.orientation
    }

    pub fn get_shield(&self) -> u8 {
        self.shield
    }

    pub fn get_invicibility(&self) -> f32 {
        self.invincibility_timer
    }

    /// Enlève un de vie
    pub fn set_shield(&mut self) {
        self.shield -= 1;
    }

    /// Mets en route l'invicibilité
    pub fn init_invicibility(&mut self) {
        self.invincibility_timer = 1.0;
    }

    /// Diminue l'invicibilité
    pub fn set_invicibility(&mut self, time: f32) {
        self.invincibility_timer -= time;
    }

    /// Fait tourner le vaisseau
    /// L (left), R (right)
    pub fn set_orientation(&mut self, dir: char) {
        match dir {
            'L' => self.orientation -= 0.03,
            'R' => self.orientation += 0.03,
            _ => panic!("pas une direction"),
        }
    }

    /// Change la vitesse du vaisseau
    pub fn set_speed(&mut self, acc: Vec2) {
        self.speed += acc;
    }

    /// Fait apparaitre le vaisseau de l'autre coté de l'écran si il sort
    pub fn bound_pos(&mut self) -> Vec2 {
        self.position.x = Self::bound_to(self.position.x, screen_width());
        self.position.y = Self::bound_to(self.position.y, screen_height());
        self.position
    }

    /// Change les coordonnées si le vaisseau sort de l'écran
    fn bound_to(coord: f32, max: f32) -> f32 {
        if coord < 0.0 {
            max - coord
        } else if coord > max {
            coord - max
        } else {
            coord
        }
    }

    /// Fait rebondir le vaisseau
    pub fn bounce(&mut self) {
        self.speed.x = -self.speed.x * 0.8;
        self.speed.y = -self.speed.y * 0.8;
    }

    /// Limiter la vitesse du vaisseau afin qu'elle ne s'arrete jamais
    pub fn min_speed(&mut self) {
        if self.speed.length() > Self::SPACESHIP_MIN_SPEED {
            self.speed *= 0.998; // on ralentit progressivement
        } else if self.speed.length() < Self::SPACESHIP_MIN_SPEED * 0.9 {
            let direction = Vec2::new(self.orientation.sin(), -self.orientation.cos()); // si trop baisse on le fait réaccélerer
            self.speed += direction * (Self::SPACESHIP_MIN_SPEED - self.speed.length());
        }
    }

    /// Limiter la vitesse du vaisseau pour qu'il n'aille pas trop vite
    pub fn max_speed(&mut self) {
        self.speed.x = self
            .speed
            .x
            .clamp(-Self::SPACESHIP_MAX_SPEED, Self::SPACESHIP_MAX_SPEED);
        self.speed.y = self
            .speed
            .y
            .clamp(-Self::SPACESHIP_MAX_SPEED, Self::SPACESHIP_MAX_SPEED);
    }

    pub fn handle_collision(&mut self, correction: Vec2) {
        if self.get_invicibility() <= 0.0 {
            self.set_shield();
            self.init_invicibility();
        }
        self.position += correction;
        self.bounce();
    }
}
