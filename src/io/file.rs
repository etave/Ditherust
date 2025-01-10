use image::{DynamicImage, ImageError};
use image::io::Reader;

pub fn open_image(image_path: &str) -> Result<DynamicImage, ImageError>{
    Reader::open(image_path)?.decode()
}

pub fn write_image(image_path: &str, image: &DynamicImage) -> Result<(), ImageError>{
    image.save(image_path)
}