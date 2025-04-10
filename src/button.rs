//! Module gérant les boutons de l'interface utilisateur.
//!
//! Ce module implémente les boutons cliquables de l'interface,
//! incluant leur affichage et leur interaction avec la souris.

use macroquad::prelude::*;

/// Représente un bouton cliquable de l'interface
///
/// # Champs
/// * `position` - Position du bouton
/// * `width` - Largeur du bouton
/// * `height` - Hauteur du bouton
/// * `color` - Couleur du bouton
/// * `text` - Texte affiché sur le bouton
pub struct Button {
    position: Vec2,
    width: f32,
    height: f32,
    color: Color,
    text: String,
}

impl Button {
    /// Crée un nouveau bouton
    pub fn new(position: Vec2, width: f32, height: f32, color: Color, text: &str) -> Self {
        Button {
            position,
            width,
            height,
            color,
            text: text.to_string(),
        }
    }

    /// Changer la couleur
    pub fn set_color(&mut self, color: Color) {
        self.color = color;
    }

    /// Dessiner un bouton avec le texte centré
    pub fn draw(&self, font: &Font) {
        draw_rectangle(
            self.position.x,
            self.position.y,
            self.width,
            self.height,
            self.color,
        );

        let text_size = measure_text(&self.text, Some(font), 20, 1.0);
        draw_text_ex(
            &self.text,
            self.position.x + (self.width - text_size.width) / 2.0,
            self.position.y + (self.height + text_size.height) / 2.0,
            TextParams {
                font_size: 20,
                font: Some(font),
                color: BLACK,
                ..Default::default()
            },
        );
    }

    /// Détecte si un bouton est survolé
    pub fn is_hovered(&self) -> bool {
        let mouse_pos = mouse_position();
        mouse_pos.0 >= self.position.x
            && mouse_pos.0 <= self.position.x + self.width
            && mouse_pos.1 >= self.position.y
            && mouse_pos.1 <= self.position.y + self.height
    }

    /// Détecte si un bouton est cliqué
    pub fn is_clicked(&self) -> bool {
        self.is_hovered() && is_mouse_button_pressed(MouseButton::Left)
    }
}
