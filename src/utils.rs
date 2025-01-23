use image::Rgba;

pub fn distance_between_colors(color1: &Rgba<u8>, color2: &Rgba<u8>) -> f64 {

    let red_diff = color1[0] as f64 - color2[0] as f64;
    let green_diff = color1[1] as f64 - color2[1] as f64;
    let blue_diff = color1[2] as f64 - color2[2] as f64;

    (red_diff.powi(2) + green_diff.powi(2) + blue_diff.powi(2)).sqrt()
}