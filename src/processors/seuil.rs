use image::{DynamicImage, GenericImage, GenericImageView, Pixel, Rgba};

pub fn seuillage(image: &mut DynamicImage) {
    let (width, height) = image.dimensions();
    for x in 0..width {
        for y in 0..height {
            let mut pixel = image.get_pixel(x, y);
            if pixel.to_luma().0[0] < 128 {
                pixel = Rgba([0, 0, 0, 255]);
            } else {
                pixel = Rgba([255, 255, 255, 255]);
            }
            image.put_pixel(x, y, pixel);
        }
    }
}