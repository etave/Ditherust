
use ditherust::handlers::{DitherustArgs, DitherustMode};
use ditherust::processors::tramage_aleatoire::tramage_aleatoire;
use image::{DynamicImage, ImageError, Rgba};
use ditherust::io::file::{open_image, write_image};
use ditherust::processors::seuil::seuillage;
use ditherust::processors::palette::palette;
use ditherust::constants::{WHITE, BLACK, BLUE, RED, GREEN, YELLOW, MAGENTA, CYAN};

fn main() -> Result<(), ImageError>{
    let args: DitherustArgs = argh::from_env();
    let path_in = args.input;

    let mut image: DynamicImage = match open_image(&path_in) {
        Ok(image) => image,
        Err(error) => return Err(error)
    };

    let output_path = match args.output {
        Some(path) => path,
        None => {
            "output.png".to_string()
        }
    };
    match args.mode {
        DitherustMode::Seuil(option) => {
            
            let couleur_1 = match option.couleur_1 {
                Some(couleur) => couleur,
                None => "000000".to_string()
            };

            let couleur_2 = match option.couleur_2 {
                Some(couleur) => couleur,
                None => "FFFFFF".to_string()
            };

            if couleur_1.len() != 6 || couleur_2.len() != 6 || !couleur_1.chars().all(|c| c.is_digit(16)) || !couleur_2.chars().all(|c| c.is_digit(16)) {
                panic!("Les couleurs doivent être des codes hexadécimaux de 6 chiffres");
            }

            seuillage(&mut image, &couleur_1, &couleur_2);

        }
        DitherustMode::Palette(option) => {

        if option.nb_couleurs < 1 || option.nb_couleurs > 9 {
            // TODO: utiliser une erreur personnalisée
            panic!("Le nombre de couleurs doit être compris entre 1 et 9");
        }

        let colors: Vec<Rgba<u8>> = vec![BLACK, WHITE, RED, GREEN, BLUE, YELLOW, CYAN, MAGENTA];        
        palette(&mut image, colors[0..option.nb_couleurs].to_vec());
        }
        DitherustMode::TramageAleatoire(_) => {
            tramage_aleatoire(&mut image);
        }
    }

    match write_image(&output_path, &image) {
        Ok(_) => {},
        Err(error) => return Err(error)
    }

    return Ok(())
}