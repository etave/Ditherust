use ditherust::constants::{BLACK, BLUE, CYAN, GREEN, MAGENTA, RED, WHITE, YELLOW};
use ditherust::handlers::{DitherustArgs, DitherustMode};
use ditherust::errors::DitherustError;
use ditherust::io::file::{open_image, write_image};
use ditherust::processors::diffusion_erreur::{diffusion_erreur, diffusion_erreur_palette};
use ditherust::processors::palette::palette;
use ditherust::processors::seuil::seuillage;
use ditherust::processors::tramage_aleatoire::tramage_aleatoire;
use ditherust::processors::tramage_bayer::tramage_bayer;
use image::{DynamicImage, Rgba};

fn main() {
    match run() {
        Ok(_) => {}
        Err(error) => eprintln!("Error: {}", error),
    }
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let args: DitherustArgs = argh::from_env();
    let path_in = args.input;

    let mut image: DynamicImage = match open_image(&path_in) {
        Ok(image) => image,
        Err(error) => return Err(Box::new(error)),
    };

    let output_path = match args.output {
        Some(path) => path,
        None => "out.png".to_string(),
    };
    match args.mode {
        DitherustMode::Seuil(option) => {
            let couleur_1 = match option.couleur_1 {
                Some(couleur) => couleur,
                None => "000000".to_string(),
            };

            let couleur_2 = match option.couleur_2 {
                Some(couleur) => couleur,
                None => "FFFFFF".to_string(),
            };

            if couleur_1.len() != 6
                || couleur_2.len() != 6
                || !couleur_1.chars().all(|c| c.is_digit(16))
                || !couleur_2.chars().all(|c| c.is_digit(16))
            {
                return Err(Box::new(DitherustError::InvalidColorFormat));
            }

            seuillage(&mut image, &couleur_1, &couleur_2);
        }
        DitherustMode::Palette(option) => {
            if option.nb_couleurs < 1 || option.nb_couleurs > 8 {
                return Err(Box::new(DitherustError::InvalidColorCount(option.nb_couleurs.try_into().unwrap())));
            }

            let colors: Vec<Rgba<u8>> = vec![BLACK, WHITE, RED, GREEN, BLUE, YELLOW, CYAN, MAGENTA];
            palette(&mut image, colors[0..option.nb_couleurs].to_vec());
        }
        DitherustMode::TramageAleatoire(_) => {
            tramage_aleatoire(&mut image);
        }
        DitherustMode::TramageBayer(option) => {
            if option.ordre > 10 {
                return Err(Box::new(DitherustError::InvalidBayerOrder(option.ordre.try_into().unwrap())));
            }

            tramage_bayer(&mut image, option.ordre);
        }
        DitherustMode::DiffusionErreur(option) => {
            match option.nb_couleurs {
                Some(nb_couleurs) => {
                    if nb_couleurs < 1 || nb_couleurs > 8 {
                        return Err(Box::new(DitherustError::InvalidColorCount(nb_couleurs.try_into().unwrap())));
                    }
        
                    let colors: Vec<Rgba<u8>> = vec![BLACK, WHITE, RED, GREEN, BLUE, YELLOW, CYAN, MAGENTA];
                    let selected_colors = colors[0..nb_couleurs].to_vec();
        
                    match option.mode.as_deref() {
                        Some("floyd-steinberg") => {
                            diffusion_erreur_palette(&mut image, selected_colors, true);
                        }
                        Some("normal") | None => {
                            diffusion_erreur_palette(&mut image, selected_colors, false);
                        }
                        Some(_) => {
                            return Err(Box::new(DitherustError::InvalidMode));
                        }
                    }
                }
                None => {
                    match option.mode.as_deref() {
                        Some("floyd-steinberg") => {
                            diffusion_erreur(&mut image, true);
                        }
                        Some("normal") | None => {
                            diffusion_erreur(&mut image, false);
                        }
                        Some(_) => {
                            return Err(Box::new(DitherustError::InvalidMode));
                        }
                    }
                }
            }
        }
    }

    match write_image(&output_path, &image) {
        Ok(_) => {}
        Err(error) => return Err(Box::new(error)),
    }

    return Ok(());
}
