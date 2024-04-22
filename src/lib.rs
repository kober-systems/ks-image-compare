use image::open;
use image::DynamicImage;
use image::GenericImageView;
use image::ImageBuffer;
use image::Rgba;
use thiserror::Error;

pub mod options;

#[derive(Error, Debug)]
pub enum Error {
    #[error(transparent)]
    ImageError(#[from] image::ImageError),
}

const EQUAL_COLOR: Rgba<u8> = Rgba([255, 255, 255, 255]);
const CHANGED_COLOR: Rgba<u8> = Rgba([255, 0, 0, 255]);

pub fn compare_images(original: &DynamicImage, compared: &DynamicImage) -> DynamicImage {
    let (width_orig, height_orig) = original.dimensions();
    let (width_comp, height_comp) = compared.dimensions();
    let width = max(width_orig, width_comp);
    let height = max(height_orig, height_comp);

    DynamicImage::ImageRgba8(ImageBuffer::from_fn(width, height, |x, y| {
        if x >= width_orig || x >= width_comp || y >= height_orig || y >= height_comp {
            CHANGED_COLOR
        } else {
            let original_pixel = original.get_pixel(x, y);
            let compared_pixel = compared.get_pixel(x, y);
            if original_pixel == compared_pixel {
                EQUAL_COLOR
            } else {
                CHANGED_COLOR
            }
        }
    }))
}

pub fn compare_images_color_difference(
    original: &DynamicImage,
    compared: &DynamicImage,
) -> DynamicImage {
    let (width, height) = original.dimensions();

    DynamicImage::ImageRgba8(ImageBuffer::from_fn(width, height, |x, y| {
        let original_pixel = original.get_pixel(x, y);
        let compared_pixel = compared.get_pixel(x, y);
        Rgba([
            rgb_channel_difference(original_pixel[0], compared_pixel[0]),
            rgb_channel_difference(original_pixel[1], compared_pixel[1]),
            rgb_channel_difference(original_pixel[2], compared_pixel[2]),
            255,
        ])
    }))
}

fn max(a: u32, b: u32) -> u32 {
    if a > b {
        a
    } else {
        b
    }
}

fn rgb_channel_difference(original: u8, compared: u8) -> u8 {
    255 - ((original as i16 - compared as i16).abs() as u8)
}

pub fn compare_images_from_path(original: &str, compared: &str) -> Result<DynamicImage, Error> {
    let original_img = open(original)?;
    let compared_img = open(compared)?;
    Ok(compare_images(&original_img, &compared_img))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_test() {
        assert_eq!(1, 1);
    }
}
