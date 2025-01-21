use image::{DynamicImage, GenericImage, GenericImageView, Rgba};


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

fn distance_between_colors(color1: &Rgba<u8>, color2: &Rgba<u8>) -> f64 {

    let red_diff = color1[0] as f64 - color2[0] as f64;
    let green_diff = color1[1] as f64 - color2[1] as f64;
    let blue_diff = color1[2] as f64 - color2[2] as f64;

    (red_diff.powi(2) + green_diff.powi(2) + blue_diff.powi(2)).sqrt()
}