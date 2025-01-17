use image::{DynamicImage, GenericImage, GenericImageView, Rgba, Pixel};
use rand::prelude::ThreadRng;
use rand::{thread_rng, Rng};

use crate::constants::{BLACK, WHITE};


pub fn tramage_aleatoire(image: &mut DynamicImage) {
    let (width, height) = image.dimensions();
    let mut rng: ThreadRng = thread_rng();
    for x in 0..width {
        for y in 0..height {
            let random_seuil: f32 = rng.gen_range(0.0..1.0);
            let mut pixel: Rgba<u8> = image.get_pixel(x, y);
            if (pixel.to_luma().0[0] as f32 / 255.0) > random_seuil {
                pixel = WHITE;
            } else {
                pixel = BLACK;
            }
            image.put_pixel(x, y, pixel);
        }
    }
}