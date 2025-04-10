//! Module gérant les projectiles tirés par le vaisseau.
//!
//! Ce module contient la logique des projectiles, incluant leur création,
//! leur déplacement et leur détection de sortie d'écran.

use macroquad::prelude::*;

/// Représente un projectile dans le jeu
///
/// # Champs
/// * `position` - Position actuelle du projectile
/// * `speed` - Vecteur vitesse du projectile
///
pub struct Bullet {
    pub position: Vec2,
    speed: Vec2,
    touched: bool,
}

impl Bullet {
    pub const BULLET_INIT_SIZE: f32 = 5.0;

    /// Crée un nouveau projectile
    pub fn new(position: Vec2, speed: Vec2) -> Self {
        Self {
            position,
            speed,
            touched: false,
        }
    }

    pub fn get_size(&self) -> f32 {
        Self::BULLET_INIT_SIZE
    }

    pub fn get_position(&self) -> Vec2 {
        self.position
    }

    pub fn get_speed(&self) -> Vec2 {
        self.speed
    }

    pub fn get_touched(&self) -> bool {
        self.touched
    }

    /// Change la valeur de touched
    pub fn set_touched(&mut self) {
        match self.touched {
            true => self.touched = false,
            false => self.touched = true,
        }
    }

    /// Vérifie si un missile est en dehors des bordures de l'écran
    pub fn is_out(&self) -> bool {
        self.position.x < 0.0
            || self.position.x > screen_width()
            || self.position.y < 0.0
            || self.position.y > screen_height()
    }

    pub fn handle_collision(&mut self) {
        self.set_touched();
    }
}
