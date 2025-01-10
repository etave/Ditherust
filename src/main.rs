
use ditherust::handlers::{DitherustArgs, DitherustMode};
use image::{DynamicImage, ImageError};
use ditherust::io::file::{open_image, write_image};
use ditherust::processors::seuil::seuillage;

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
        DitherustMode::Seuil(_) => seuillage(&mut image),
        DitherustMode::Palette(_) => {}
    }

    match write_image(&output_path, &image) {
        Ok(_) => {},
        Err(error) => return Err(error)
    }

    return Ok(())
}