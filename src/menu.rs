//! Module représentant le menu principal du jeu
//!
//! Ce module contient la structure `Menu` qui gère l'affichage et les interactions du menu principal.

use crate::button::Button;
use crate::Difficulty;
use macroquad::prelude::*;

/// Représente le menu principal du jeu
///
/// # Champs
/// * `btn_easy` - Bouton pour le mode facile
/// * `btn_medium` - Bouton pour le mode moyen
/// * `btn_hard` - Bouton pour le mode difficile
/// * `btn_quit` - Bouton pour quitter le jeu
/// * `difficulty` - Difficulté sélectionnée par le joueur
pub struct Menu {
    btn_easy: Button,
    btn_medium: Button,
    btn_hard: Button,
    btn_test: Button,
    btn_quit: Button,
    difficulty: Option<Difficulty>,
}

impl Menu {
    /// Crée une nouvelle instance du menu principal
    ///
    /// # Retourne
    /// Une nouvelle instance de Menu avec les boutons positionnés
    pub fn new() -> Self {
        let screen_center_x = screen_width() / 2.0;
        let button_width = 200.0;
        let button_height = 40.0;
        let spacing = 60.0;

        Self {
            btn_easy: Button::new(
                Vec2::new(screen_center_x - button_width / 2.0, 200.0),
                button_width,
                button_height,
                GREEN,
                "Facile",
            ),
            btn_medium: Button::new(
                Vec2::new(screen_center_x - button_width / 2.0, 200.0 + spacing),
                button_width,
                button_height,
                YELLOW,
                "Moyen",
            ),
            btn_hard: Button::new(
                Vec2::new(screen_center_x - button_width / 2.0, 200.0 + spacing * 2.0),
                button_width,
                button_height,
                RED,
                "Difficile",
            ),
            btn_test: Button::new(
                Vec2::new(screen_center_x - button_width / 2.0, 200.0 + spacing * 3.0),
                button_width,
                button_height,
                BLUE,
                "Test vaisseau",
            ),
            btn_quit: Button::new(
                Vec2::new(screen_center_x - button_width / 2.0, 200.0 + spacing * 4.0),
                button_width,
                button_height,
                GRAY,
                "Quitter",
            ),
            difficulty: None,
        }
    }

    /// Dessine le menu et gère les interactions
    ///
    /// # Arguments
    /// * `font` - Police de caractères à utiliser pour le texte
    /// * `background` - Texture d'arrière-plan à afficher
    pub fn draw(&mut self, font: &Font, background: &Texture2D) {
        // Dessiner le fond
        draw_texture(background, 0., 0., WHITE);

        // Dessiner le titre
        let title = "ASTEROIDS";

        let title_size = measure_text(title, Some(font), 60, 1.0);
        draw_text_ex(
            title,
            screen_width() / 2.0 - title_size.width / 2.0,
            100.0,
            TextParams {
                font_size: 60,
                font: Some(font),
                color: WHITE,
                ..Default::default()
            },
        );

        // Dessiner les boutons
        self.btn_easy.draw(font);
        self.btn_medium.draw(font);
        self.btn_hard.draw(font);
        self.btn_test.draw(font);
        self.btn_quit.draw(font);

        // Gérer les survols
        if self.btn_quit.is_hovered() {
            self.btn_quit.set_color(DARKGRAY);
        } else {
            self.btn_quit.set_color(GRAY);
        }

        // Gérer les clics
        if self.btn_easy.is_clicked() {
            self.difficulty = Some(Difficulty::Easy);
        } else if self.btn_medium.is_clicked() {
            self.difficulty = Some(Difficulty::Medium);
        } else if self.btn_hard.is_clicked() {
            self.difficulty = Some(Difficulty::Hard);
        }
    }

    /// Récupère la difficulté sélectionnée
    ///
    /// # Retourne
    /// * `Some(Difficulty)` si une difficulté a été sélectionnée
    /// * `None` si aucune difficulté n'a encore été choisie
    pub fn get_difficulty(&self) -> Option<Difficulty> {
        self.difficulty
    }

    /// Vérifie si le bouton quitter a été cliqué
    ///
    /// # Retourne
    /// `true` si le bouton quitter a été cliqué, `false` sinon
    pub fn quit_clicked(&self) -> bool {
        self.btn_quit.is_clicked()
    }

    pub fn test_clicked(&self) -> bool {
        self.btn_test.is_clicked()
    }
}
