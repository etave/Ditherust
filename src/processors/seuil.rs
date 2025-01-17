use image::{DynamicImage, GenericImage, GenericImageView, Pixel, Rgba};
use hex;


pub fn seuillage(image: &mut DynamicImage, color_1: &str, color_2: &str) {

    let replacement_color_1: Rgba<u8> = hexa_to_rgba(color_1);
    let replacement_color_2: Rgba<u8> = hexa_to_rgba(color_2);

    let (width, height) = image.dimensions();
    for x in 0..width {
        for y in 0..height {
            let mut pixel: Rgba<u8> = image.get_pixel(x, y);

            if pixel.to_luma().0[0] < 128 {
                pixel = replacement_color_1;
            } else {
                pixel = replacement_color_2;
            }
            image.put_pixel(x, y, pixel);
        }
    }
}

fn hexa_to_rgba(hexa: &str) -> Rgba<u8> {
    let r: u8 = hex::decode(hexa[0..2].to_string()).unwrap()[0];
    let g: u8 = hex::decode(hexa[2..4].to_string()).unwrap()[0];
    let b: u8 = hex::decode(hexa[4..6].to_string()).unwrap()[0];
    Rgba([r, g, b, 255])
}