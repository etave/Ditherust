use image::{DynamicImage, GenericImage, GenericImageView, Pixel, Rgba};

use crate::constants::{BLACK, WHITE};

fn generate_bayer_matrix(level: u32) -> Vec<Vec<f32>> {
    if level == 0 {
        return vec![vec![0.0]];
    }
    if level == 1 {
        return vec![vec![0.0, 2.0], vec![3.0, 1.0]];
    }

    let half_size: u32 = 2_u32.pow(level - 1);
    let sub_matrix: Vec<Vec<f32>> = generate_bayer_matrix(level - 1);

    let size: u32 = 2_u32.pow(level);
    let mut matrix: Vec<Vec<f32>> = vec![vec![0.0; size as usize]; size as usize];
    let n_squared: f32 = (size * size) as f32;

    for i in 0..half_size {
        for j in 0..half_size {
            matrix[i as usize][j as usize] = 4.0 * sub_matrix[i as usize][j as usize];
            matrix[i as usize][(j + half_size) as usize] =
                4.0 * sub_matrix[i as usize][j as usize] + 2.0;
            matrix[(i + half_size) as usize][j as usize] =
                4.0 * sub_matrix[i as usize][j as usize] + 3.0;
            matrix[(i + half_size) as usize][(j + half_size) as usize] =
                4.0 * sub_matrix[i as usize][j as usize] + 1.0;
        }
    }

    for i in 0..size {
        for j in 0..size {
            matrix[i as usize][j as usize] /= n_squared;
        }
    }

    matrix
}

pub fn tramage_bayer(image: &mut DynamicImage, level: u32) {
    let bayer_matrix: Vec<Vec<f32>> = generate_bayer_matrix(level);
    let bayer_size: u32 = bayer_matrix.len() as u32;

    let (width, height) = image.dimensions();
    for y in 0..height {
        for x in 0..width {
            let mut pixel: Rgba<u8> = image.get_pixel(x, y);
            if pixel.to_luma().0[0] as f32
                > bayer_matrix[(x % bayer_size) as usize][(y % bayer_size) as usize] * 255.0
            {
                pixel = WHITE;
            } else {
                pixel = BLACK;
            }
            image.put_pixel(x, y, pixel);
        }
    }
}
