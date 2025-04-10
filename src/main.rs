//! Module principal du jeu Asteroids.
//!
//! Ce module contient la boucle principale du jeu et gère :
//! * L'affichage des éléments graphiques
//! * Les interactions utilisateur
//! * La logique de collision
//! * La gestion des états du jeu (victoire/défaite)

use asteroid::Asteroid;
use bullet::Bullet;
use macroquad::prelude::*;
use menu::Menu;
use spaceship::Spaceship;
use stellarobject::StellarObject;

mod asteroid;
mod bullet;
mod button;
mod menu;
mod spaceship;
mod stellarobject;

/// Niveaux de difficulté du jeu
///
/// # Variantes
/// * `Easy` - Mode facile avec 3 astéroïdes
/// * `Medium` - Mode moyen avec 6 astéroïdes
/// * `Hard` - Mode difficile avec 10 astéroïdes
#[derive(Debug, Clone, Copy)]
pub enum Difficulty {
    Easy,
    Medium,
    Hard,
}

/// Fonction principale de rendu graphique
///
/// # Arguments
/// * `background` - Texture de l'arrière-plan
/// * `asteroids` - Vecteur contenant tous les astéroïdes
/// * `spaceship` - Instance du vaisseau spatial
/// * `spaceship_skin` - Image du vaisseau
/// * `parametres` - Parametres liés à l'affichage du vaisseau
/// * `asteroid_skin` - Images des astéroides
/// * `parametres2` - Parametres liés à l'affichage des asteroides
/// * `bullets` - Vecteur contenant tous les projectiles
/// * `shield` - Points de bouclier actuels
/// * `font` - Police de caractères pour le texte
fn draw(
    background: &Texture2D,
    spaceship: &Spaceship,
    spaceship_skin: &Texture2D,
    parametres: &DrawTextureParams,
    asteroids: &Vec<Asteroid>,
    asteroid_skin: &Texture2D,
    parametres2: &DrawTextureParams,
    bullets: &Vec<Bullet>,
    shield: &u8,
    font: &Font,
) {
    // affichage du fond d'écran
    draw_background(background);

    // affichage de l'image du vaisseau
    draw_spaceship_skin(
        spaceship_skin,
        spaceship.get_position().x - 35.,
        spaceship.get_position().y - 30.,
        parametres,
    );

    // On affiche l'image de tous les asteroides en fonction de leur tailles
    for a in asteroids {
        // Selon la taille du l'asteroide (ou de la "hitbox"), on modifie la taille et la position de l'image de l'asteroide
        // de manière à ce que ces paramètres soient identique à ceux de la "hitbox"
        if a.get_width() == 25. {
            let new_parametres2 = DrawTextureParams {
                dest_size: Some(vec2(50.0, 50.0)), // changement de la taille du vaisseau
                ..Default::default()
            };
            draw_asteroid_skin(
                asteroid_skin,
                a.get_position().x - 25., // changement de la position de l'image par rapport à sa "hitbox"
                a.get_position().y - 25.,
                &new_parametres2,
            );

            // si la taille de la "hitbox" de l'asteroide est 15, on modifie la taille de l'image de l'asteroide
        } else if a.get_width() == 15. {
            let new_parametres2 = DrawTextureParams {
                dest_size: Some(vec2(30.0, 30.0)), // changement de la taille du vaisseau
                ..Default::default()
            };
            draw_asteroid_skin(
                asteroid_skin,
                a.get_position().x - 15., // changement de la position de l'image par rapport à sa "hitbox"
                a.get_position().y - 15.,
                &new_parametres2,
            );
        } else {
            // si l'asteroide est de taille initial (50), on garde la taille définit dans la fonction main
            draw_asteroid_skin(
                asteroid_skin,
                a.get_position().x - 50.,
                a.get_position().y - 50.,
                parametres2,
            );
        }
    }

    for b in bullets {
        draw_bullet(b);
    }
    draw_shield(shield, font);
}

/// Fonction de rendu graphique pour le test 1
/// Test 1 : Affichage d'un vaisseau et d'un missile
///
/// # Arguments
/// * `background` - Texture de l'arrière-plan
/// * `spaceship` - Instance du vaisseau spatial
/// * `spaceship_skin` - Texture du vaisseau
/// * `parametres` - Paramètres d'affichage du vaisseau
/// * `bullets` - Vecteur contenant tous les projectiles    
fn draw_test_1(
    background: &Texture2D,
    spaceship: &Spaceship,
    spaceship_skin: &Texture2D,
    parametres: &DrawTextureParams,
    bullets: &Vec<Bullet>,
) {
    draw_background(background);

    draw_spaceship_skin(
        spaceship_skin,
        spaceship.get_position().x - 35.,
        spaceship.get_position().y - 30.,
        parametres,
    );

    for b in bullets {
        draw_bullet(b);
    }
}

/// Dessine l'arrière-plan du jeu
///
/// # Arguments
/// * `background` - Texture à utiliser pour l'arrière-plan
fn draw_background(background: &Texture2D) {
    draw_texture(background, 0., 0., WHITE);
}

/// Dessine l'image du vaisseau
///
/// # Arguments
/// * `spaceship_skin` - Texture du vaisseau à dessiner
/// * `pos_x` - Position x du vaisseau
/// * `pos_y` - Position y du vaisseau
/// * `params` - Paramètres d'affichage du vaisseau
fn draw_spaceship_skin(
    spaceship_skin: &Texture2D,
    pos_x: f32,
    pos_y: f32,
    params: &DrawTextureParams,
) {
    draw_texture_ex(spaceship_skin, pos_x, pos_y, WHITE, params.clone());
}

/// Dessine l'image de l'asteroide
///
/// # Arguments
/// * `asteroid_skin` - Texture de l'asteroide à dessiner
/// * `pos_x` - Position x de l'asteroide
/// * `pos_y` - Position y de l'asteroide
/// * `params` - Paramètres d'affichage de l'asteroide  
fn draw_asteroid_skin(
    asteroid_skin: &Texture2D,
    pos_x: f32,
    pos_y: f32,
    params: &DrawTextureParams,
) {
    draw_texture_ex(asteroid_skin, pos_x, pos_y, WHITE, params.clone());
}

/// Dessine un projectile sous forme de cercle
///
/// # Arguments
/// * `bullet` - Instance du projectile à dessiner
fn draw_bullet(bullet: &Bullet) {
    draw_circle(
        bullet.get_position().x,
        bullet.get_position().y,
        bullet.get_size(),
        RED,
    );
}

/// Affiche la barre de bouclier du vaisseau
///
/// # Arguments
/// * `shield` - Nombre de points de bouclier restants
/// * `font` - Police de caractères à utiliser
fn draw_shield(shield: &u8, font: &Font) {
    let mut text = String::from("Bouclier : ");
    for _i in 0..*shield {
        text.push('|');
    }
    // let text_dimensions = measure_text(&text, Some(font), 20, 1.0);
    draw_text_ex(
        &text,
        screen_width() - 180.,
        screen_height() - 30.,
        TextParams {
            font_size: 20,
            font: Some(font),
            color: RED,
            ..Default::default()
        },
    );
}

/// Initialise le vecteur d'astéroïdes au début du jeu
///
/// # Arguments
/// * `difficulty` - Niveau de difficulté choisi qui détermine le nombre d'astéroïdes
///
/// # Retourne
/// Un vecteur contenant les astéroïdes initiaux selon la difficulté choisie
///
/// # Exemple
/// ```
/// let asteroids = create_asteroids(Difficulty::Easy); // Crée 3 astéroïdes
/// assert_eq!(asteroids.len(), 3);
/// ```
fn create_asteroids(difficulty: Difficulty) -> Vec<Asteroid> {
    let asteroid_count = match difficulty {
        Difficulty::Easy => 3,
        Difficulty::Medium => 6,
        Difficulty::Hard => 10,
    };

    (0..asteroid_count)
        .map(|_| asteroid::Asteroid::new())
        .collect()
}

/// Gère la division d'un astéroïde après une collision
/// Crée deux astéroïdes fils après la division d'un astéroïde parent
///
/// # Arguments
/// * `position` - La position de l'astéroïde à diviser
/// * `width` - Taille de l'astéroïde à diviser
///
/// # Retourne
/// Deux astéroïdes de tailles inférieurs ou erreur
///
/// # Exemple
/// ```
/// let (a1, a2) = divide(Vec2::new(0., 0.), 50.0).unwrap();
/// assert_eq!(a1.get_width(), 25.0);
/// assert_eq!(a2.get_width(), 25.0);
/// ```
fn divide(position: Vec2, width: f32) -> Result<(Asteroid, Asteroid), String> {
    match width {
        25.0 => Ok((
            Asteroid::new2(vec2(position.x + 1., position.y + 1.), 15.0),
            Asteroid::new2(vec2(position.x - 1., position.y - 1.), 15.0),
        )),
        50.0 => Ok((
            Asteroid::new2(vec2(position.x + 1., position.y + 1.), 25.0),
            Asteroid::new2(vec2(position.x - 1., position.y - 1.), 25.0),
        )),
        _ => Err("Taille non gérée".to_string()),
    }
}

/// Point d'entrée principal du jeu
///
/// Cette fonction contient la boucle principale du jeu et gère :
/// * L'initialisation des ressources
/// * La boucle du menu
/// * La boucle de jeu
/// * Les entrées utilisateur
/// * La physique du jeu
/// * Les conditions de victoire/défaite
#[macroquad::main("BasicShapes")]
async fn main() {
    // Chargement de la police d'ecriture
    let font = load_ttf_font("./font/BebasNeue-Regular.ttf").await.unwrap();

    // Chargement des différents assets
    set_pc_assets_folder("img");
    let background_texture = load_texture("bg_3.png").await.expect("Couldn't load file");
    let spaceship_texture = load_texture("image_vaisseau.png").await.unwrap();
    let asteroid_texture = load_texture("image_asteroide.png").await.unwrap();

    // Initialisation des différentes composantes du jeu
    let mut asteroids: Vec<Asteroid> = Vec::new();
    let mut spaceship = spaceship::Spaceship::new();
    let mut bullets: Vec<Bullet> = Vec::new();
    let mut gameover = false;
    let mut last_shot = get_time();

    // Paramètres d'affichage du vaisseau
    let mut parametres = DrawTextureParams {
        dest_size: Some(vec2(70.0, 60.0)), // Redimensionner l'image à 200x200 pixels
        rotation: spaceship.get_orientation(), // Rotation en radians (ici, 45 degrés)
        pivot: Some(spaceship.get_position()), // Afficher l'image entière (pas de découpe)
        ..Default::default()               // Autres paramètres par défaut
    };

    let mut parametres2 = DrawTextureParams {
        dest_size: Some(vec2(100.0, 100.0)), // Redimensionner l'image à 200x200 pixels
        ..Default::default()                 // Autres paramètres par défaut
    };

    // Création des boutons 'quitter' et 'rejouer'
    let mut btn_quit = button::Button::new(Vec2::new(20., 20.), 70.0, 20.0, BLACK, "Quitter");
    let mut btn_replay = button::Button::new(
        Vec2::new(screen_width() - 90., 20.),
        70.0,
        20.0,
        BLACK,
        "Rejouer",
    );

    // Création du menu
    let mut menu = Menu::new();
    let mut in_menu = true;
    let mut test_1 = false;

    loop {
        // Boucle du menu, on attend la difficulté choisie par le joueur
        if in_menu {
            // Quitter le jeu à n'importe quel moment
            if is_key_down(KeyCode::Escape) || menu.quit_clicked() {
                break;
            }

            if menu.test_clicked() {
                test_1 = true;
                in_menu = false;
            }

            menu.draw(&font, &background_texture);

            if let Some(difficulty) = menu.get_difficulty() {
                asteroids = create_asteroids(difficulty);
                in_menu = false;
                test_1 = false;
            }

            next_frame().await;
            continue;
        }

        // Boucle du test du vaisseau
        if test_1 {
            let frame_t = get_time();

            // Mise à jour des paramètres d'affichage du vaisseau
            parametres.rotation = spaceship.get_orientation();
            parametres.pivot = Some(spaceship.get_position());

            draw_test_1(
                &background_texture,
                &spaceship,
                &spaceship_texture,
                &parametres,
                &bullets,
            );
            btn_quit.draw(&font);
            btn_replay.draw(&font);

            if is_key_down(KeyCode::Escape) {
                break;
            }

            // Gestion du bouton 'quitter'
            if btn_quit.is_clicked() {
                break;
            } else if btn_quit.is_hovered() {
                btn_quit.set_color(GRAY);
            } else {
                btn_quit.set_color(RED);
            }

            // Gestion du bouton 'rejouer' qui ramène vers le menu
            if btn_replay.is_clicked() {
                spaceship = spaceship::Spaceship::new();
                asteroids.clear();
                bullets.clear();
                in_menu = true;
                menu = Menu::new();
                gameover = false;
            } else if btn_replay.is_hovered() {
                btn_replay.set_color(GRAY);
            } else {
                btn_replay.set_color(RED);
            }

            // Tourner (orientation)
            if is_key_down(KeyCode::Right) {
                spaceship.set_orientation('R');
            } else if is_key_down(KeyCode::Left) {
                spaceship.set_orientation('L');
            }

            // Faire en sorte que le vaisseau ne s'arrête jamais
            // si aucune touche n'est pressée
            if !is_key_down(KeyCode::Up)
                && !is_key_down(KeyCode::Down)
                && !is_key_down(KeyCode::Left)
                && !is_key_down(KeyCode::Right)
            {
                spaceship.min_speed();
            }

            // Avancer
            if is_key_down(KeyCode::Up) {
                let acc = Vec2::new(
                    spaceship.get_orientation().sin(),
                    -spaceship.get_orientation().cos(),
                ) / 30.0;
                spaceship.set_speed(acc);
            } else if is_key_down(KeyCode::Down) {
                let acc = Vec2::new(
                    -spaceship.get_orientation().sin(),
                    spaceship.get_orientation().cos(),
                ) / 40.0;
                spaceship.set_speed(acc);
            }

            //On deplace la position du vaisseau
            spaceship.move_object();
            spaceship.max_speed(); //force a ne pas depasser le vmax

            // Fait réapparaitre le vaisseau à l'opposé de l'ecran lorsque l'on touche une limite d'ecran
            spaceship.bound_pos();

            // Lancer un missile si la touche espace est pressée et que le temps entre chaque tir est supérieur à 0.5s
            if is_key_down(KeyCode::Space) && frame_t - last_shot > 0.5 {
                // Calcul de la direction du missile
                let rot_vec = Vec2::new(
                    spaceship.get_orientation().sin(),
                    -spaceship.get_orientation().cos(),
                );
                bullets.push(bullet::Bullet::new(
                    spaceship.get_position() + rot_vec * spaceship.get_height() / 2.,
                    rot_vec * 4.,
                ));
                last_shot = frame_t;
            }

            // Bouger un missile
            for bullet in bullets.iter_mut() {
                bullet.move_object();
            }

            // Si le missile dépasse les bords de l'écran il disparait
            bullets.retain(|bullet| !bullet.is_out());

            next_frame().await;
            continue;
        }

        // Boucle pour le jeu
        if !in_menu {
            let frame_t = get_time();

            // Fait diminuer le temps d'invicibilité à chaque frame
            if spaceship.get_invicibility() > 0.0 {
                spaceship.set_invicibility(get_frame_time());
            }

            // Mise à jour des paramètres d'affichage du vaisseau
            parametres.rotation = spaceship.get_orientation();
            parametres.pivot = Some(spaceship.get_position());

            // Mise à jour des paramètres d'affichage des asteroides
            parametres2.dest_size = Some(vec2(100.0, 100.0));

            // Pour afficher les différents objets du jeu
            draw(
                &background_texture,
                &spaceship,
                &spaceship_texture,
                &parametres,
                &asteroids,
                &asteroid_texture,
                &parametres2,
                &bullets,
                &spaceship.get_shield(),
                &font,
            );

            btn_quit.draw(&font);
            btn_replay.draw(&font);

            // Quitter le jeu à n'importe quel moment
            if is_key_down(KeyCode::Escape) {
                break;
            }

            // Gestion du bouton 'quitter'
            if btn_quit.is_clicked() {
                break;
            } else if btn_quit.is_hovered() {
                btn_quit.set_color(GRAY);
            } else {
                btn_quit.set_color(RED);
            }

            // Gestion du bouton 'rejouer'
            if btn_replay.is_clicked() {
                spaceship = spaceship::Spaceship::new();
                asteroids.clear();
                bullets.clear();
                in_menu = true;
                menu = Menu::new();
                gameover = false;
            } else if btn_replay.is_hovered() {
                btn_replay.set_color(GRAY);
            } else {
                btn_replay.set_color(RED);
            }

            // On fait bouger tous les asteroides encore présent et remets leur collision à faux
            for asteroid in &mut asteroids {
                asteroid.set_collision(false);
                asteroid.move_object();
                asteroid.bound_pos();
            }

            // Tourner (orientation)
            if is_key_down(KeyCode::Right) {
                spaceship.set_orientation('R');
            } else if is_key_down(KeyCode::Left) {
                spaceship.set_orientation('L');
            }

            // Faire en sorte que le vaisseau ne s'arrête jamais
            // si aucune touche n'est pressée
            if !is_key_down(KeyCode::Up)
                && !is_key_down(KeyCode::Down)
                && !is_key_down(KeyCode::Left)
                && !is_key_down(KeyCode::Right)
            {
                spaceship.min_speed();
            }

            // Avancer
            if is_key_down(KeyCode::Up) {
                let acc = Vec2::new(
                    spaceship.get_orientation().sin(),
                    -spaceship.get_orientation().cos(),
                ) / 30.0;
                spaceship.set_speed(acc);
            } else if is_key_down(KeyCode::Down) {
                let acc = Vec2::new(
                    -spaceship.get_orientation().sin(),
                    spaceship.get_orientation().cos(),
                ) / 40.0;
                spaceship.set_speed(acc);
            }

            //On deplace la position du vaisseau
            spaceship.move_object();
            spaceship.max_speed(); //force a ne pas depasser le vmax

            // Fait réapparaitre le vaisseau à l'opposé de l'ecran lorsque l'on touche une limite d'ecran
            spaceship.bound_pos();

            // Lancer un missile si la touche espace est pressée et que le temps entre chaque tir est supérieur à 0.5s
            if is_key_down(KeyCode::Space) && frame_t - last_shot > 0.5 {
                // Calcul de la direction du missile
                let rot_vec = Vec2::new(
                    spaceship.get_orientation().sin(),
                    -spaceship.get_orientation().cos(),
                );
                bullets.push(bullet::Bullet::new(
                    spaceship.get_position() + rot_vec * spaceship.get_height() / 2.,
                    rot_vec * 4.,
                ));
                last_shot = frame_t;
            }

            // Bouger un missile
            for bullet in bullets.iter_mut() {
                bullet.move_object();
            }

            // Si le missile dépasse les bords de l'écran il disparait
            bullets.retain(|bullet| !bullet.is_out());

            // Collision astéroide / astéroide
            for i in 0..asteroids.len() {
                for j in (i + 1)..asteroids.len() {
                    let (asteroid1, asteroid2) = {
                        let (left, right) = asteroids.split_at_mut(j);
                        (&mut left[i], &mut right[0])
                    };
                    asteroid1.collided(asteroid2);
                }
            }

            // Collision entre un astéroide et le vaisseau
            for asteroid in asteroids.iter_mut() {
                spaceship.collided(asteroid);
            }

            // Check la vie restante du vaisseau
            if spaceship.get_shield() == 0 {
                gameover = true;
            }

            // Collision missile avec un astéroide
            let mut new_asteroids: Vec<Asteroid> = Vec::new();
            for asteroid in asteroids.iter_mut() {
                for bullet in bullets.iter_mut() {
                    if bullet.collided(asteroid) && asteroid.get_width() > 15.0 {
                        // On unwrap car l'erreur ne peut pas se produire
                        let (a1, a2) =
                            divide(asteroid.get_position(), asteroid.get_width()).unwrap();
                        new_asteroids.push(a1);
                        new_asteroids.push(a2);
                    }
                }
            }

            asteroids.append(&mut new_asteroids);

            // Supprime les astéroides et les projectiles qui ont été touchés
            asteroids.retain(|asteroid| !asteroid.get_touched());
            bullets.retain(|bullet| !bullet.get_touched());

            // Regarde si la partie est gagné ou perdu
            if gameover || asteroids.is_empty() {
                asteroids.clear();
                bullets.clear(); // Reset

                let mut text = "Gagné ! Presser 'Entrée' pour rejouer";
                let mut font_color = GREEN;
                if gameover {
                    text = "Perdu ! Presser 'Entrée' pour rejouer";
                    font_color = RED;
                }
                let text_dimensions = measure_text(text, Some(&font), 40, 1.0);
                draw_text_ex(
                    text,
                    screen_width() / 2. - text_dimensions.width / 2.0,
                    screen_height() / 2.,
                    TextParams {
                        font_size: 40,
                        font: Some(&font),
                        color: font_color,
                        ..Default::default()
                    },
                );

                // Rejouer
                if is_key_down(KeyCode::Enter) {
                    asteroids = create_asteroids(Difficulty::Easy);
                    spaceship = spaceship::Spaceship::new();
                    gameover = false;
                    continue;
                }
            }

            next_frame().await
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    // j#[test]
    // // peut pas foncionner car on a besoin de la taillle de l'ecran, il faut etre dans un boucle avec une window
    // fn creation_asteroide() {
    //     let asteroids = create_asteroids(Difficulty::Easy);
    //     assert_eq!(asteroids.len(), 3);
    // }

    #[test]
    fn division_asteroide() {
        let (a1, a2) = divide(vec2(0., 0.), 50.0).unwrap();
        assert_eq!(a1.get_width(), 25.0);
        assert_eq!(a2.get_width(), 25.0);
    }
}
