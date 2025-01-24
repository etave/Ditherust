use image::{DynamicImage, GenericImage, GenericImageView, Rgba};
use crate::utils::distance_between_colors;

pub fn palette(image: &mut DynamicImage, colors: Vec<Rgba<u8>>) {
    let (width, height) = image.dimensions();
    for x in 0..width {
        for y in 0..height {
            let pixel = image.get_pixel(x, y);
            let mut closest_color = colors[0];
            let mut min_distance = distance_between_colors(&pixel, &closest_color);
            for color in &colors {
                let distance = distance_between_colors(&pixel, color);
                if distance < min_distance {
                    min_distance = distance;
                    closest_color = *color;
                }
            }
            image.put_pixel(x, y, closest_color);
        }
    }
}