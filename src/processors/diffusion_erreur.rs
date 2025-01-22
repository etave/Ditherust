use image::{DynamicImage, GenericImage, GenericImageView, Rgba};

use crate::constants::{BLACK, WHITE};

pub fn diffusion_erreur(image: &mut DynamicImage) {
    let (width, height) = image.dimensions();

    for y in 0..height {
        for x in 0..width {
            let old_pixel = image.get_pixel(x, y);
            let new_pixel = if old_pixel[0] > 128 { WHITE } else { BLACK };
            let error = calculate_error(&old_pixel, new_pixel);

            image.put_pixel(x, y, new_pixel);

            distribute_error(image, x, y, width, height, error);
        }
    }
}

fn calculate_error(old_pixel: &Rgba<u8>, new_pixel: Rgba<u8>) -> [i16; 3] {
    [
        old_pixel[0] as i16 - new_pixel[0] as i16,
        old_pixel[1] as i16 - new_pixel[1] as i16,
        old_pixel[2] as i16 - new_pixel[2] as i16,
    ]
}

fn distribute_error(
    image: &mut DynamicImage,
    x: u32,
    y: u32,
    width: u32,
    height: u32,
    error: [i16; 3],
) {
    if x + 1 < width {
        add_error_to_pixel(image, x + 1, y, error, 0.5);
    }

    if y + 1 < height {
        add_error_to_pixel(image, x, y + 1, error, 0.5);
    }
}

fn add_error_to_pixel(image: &mut DynamicImage, x: u32, y: u32, error: [i16; 3], factor: f32) {
    let pixel = image.get_pixel(x, y);
    let new_pixel = [
        (pixel[0] as f32 + error[0] as f32 * factor).clamp(0.0, 255.0) as u8,
        (pixel[1] as f32 + error[1] as f32 * factor).clamp(0.0, 255.0) as u8,
        (pixel[2] as f32 + error[2] as f32 * factor).clamp(0.0, 255.0) as u8,
        pixel[3],
    ];
    image.put_pixel(x, y, Rgba(new_pixel));
}
