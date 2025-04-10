//! Module gérant les astéroïdes du jeu.
//!
//! Ce module implémente la logique des astéroïdes, incluant leur création,
//! leur mouvement et leur comportement lors des collisions.

use std::f32::consts::PI;

use ::rand::{thread_rng, Rng};
use macroquad::prelude::*;

/// Représente un astéroïde dans le jeu
///
/// # Champs
/// * `position` - Position actuelle de l'astéroïde
/// * `speed` - Vecteur vitesse de l'astéroïde
/// * `width` - Rayon de l'astéroïde
/// * `collided` - État de collision de l'astéroïde
/// * `touched` - État de touché de l'astéroïde
pub struct Asteroid {
    pub position: Vec2,
    speed: Vec2,
    width: f32,
    collided: bool,
    touched: bool,
}

impl Asteroid {
    /// Taille initiale d'un astéroïde nouvellement créé
    pub const ASTEROID_INIT_SIZE: f32 = 50.0;

    /// Crée un nouvel astéroïde avec une position et une vitesse aléatoires
    pub fn new() -> Self {
        Self {
            position: Self::new_alea_pos(),
            speed: Self::new_alea_speed(),
            width: Asteroid::ASTEROID_INIT_SIZE,
            collided: false,
            touched: false,
        }
    }

    /// Crée un nouvel astéroïde avec une vitesse aléatoires, une position et une taille donné
    pub fn new2(position: Vec2, width: f32) -> Self {
        Self {
            position,
            speed: Self::new_alea_speed(),
            width,
            collided: false,
            touched: false,
        }
    }

    pub fn get_position(&self) -> Vec2 {
        self.position
    }

    pub fn get_width(&self) -> f32 {
        self.width
    }

    pub fn get_speed(&self) -> Vec2 {
        self.speed
    }

    pub fn get_touched(&self) -> bool {
        self.touched
    }

    /// Change la valeur de collided
    pub fn set_collision(&mut self, bool: bool) {
        self.collided = bool;
    }

    /// Change la valeur de touched
    pub fn set_touched(&mut self) {
        match self.touched {
            true => self.touched = false,
            false => self.touched = true,
        }
    }

    /// Génère une position aléatoire près de l'un des bords.
    fn new_alea_pos() -> Vec2 {
        let mut rng = thread_rng();

        let nearpos: f32 = rng.gen_range(Self::ASTEROID_INIT_SIZE / 2.0..=Self::ASTEROID_INIT_SIZE);
        let nearside = rng.gen_range(1..=4); // 1 = top, 2 = right, 3 = down, 4 = left
        let xpos: f32 = match nearside {
            2 => screen_width() - nearpos,
            4 => nearpos,
            _ => rng.gen_range(0.0..=screen_width()),
        };
        let ypos: f32 = match nearside {
            1 => nearpos,
            3 => screen_height() - nearpos,
            _ => rng.gen_range(0.0..=screen_height()),
        };
        vec2(xpos, ypos)
    }

    /// Génère une direction aléatoire
    /// Si l'angle = 0: vers le droite
    ///            = π/2: vers le haut
    ///            = π: vers le gauche
    ///            = 3π/2: vers le bas
    fn new_alea_speed() -> Vec2 {
        let mut rng = thread_rng();

        let angle: f32 = rng.gen_range(0.0..=2.0 * PI);
        Vec2::from_angle(angle)
    }

    /// Fait apparaitre l'astéroïde de l'autre coté de l'écran si il sort
    pub fn bound_pos(&mut self) -> Vec2 {
        self.position.x = Self::bound_to(self.position.x, screen_width());
        self.position.y = Self::bound_to(self.position.y, screen_height());
        self.position
    }

    /// Change les coordonnées si l'astéroïde sort de l'écran
    fn bound_to(coord: f32, max: f32) -> f32 {
        if coord < 0.0 {
            max - coord
        } else if coord > max {
            coord - max
        } else {
            coord
        }
    }

    /// Fait rebondir l'astéroide
    pub fn bounce(&mut self) {
        self.speed.x = -self.speed.x;
        self.speed.y = -self.speed.y;
    }

    pub fn handle_collision(&mut self, correction: Vec2) {
        self.position += correction;
        self.bounce();
    }
}
