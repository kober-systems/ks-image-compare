use image::open;
use image::DynamicImage;
use image::GenericImageView;
use image::ImageBuffer;
use image::Rgba;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
  #[error(transparent)]
  ImageError(#[from] image::ImageError),
}

const EQUAL_COLOR: Rgba<u8> = Rgba([255, 255, 255, 255]);
const CHANGED_COLOR: Rgba<u8> = Rgba([255, 0, 0, 255]);

fn compare_images(original: DynamicImage, compared: DynamicImage) -> DynamicImage {
    let (width, height) = original.dimensions();

    DynamicImage::ImageRgba8(ImageBuffer::from_fn(width, height, |x, y| {
        let original_pixel = original.get_pixel(x, y);
        let compared_pixel = compared.get_pixel(x, y);
        if original_pixel == compared_pixel {
            EQUAL_COLOR
        } else {
            CHANGED_COLOR
        }
    }))
}

pub fn compare_images_from_path(original: &str, compared: &str) -> Result<DynamicImage, Error> {
    let original_img = open(original)?;
    let compared_img = open(compared)?;
    Ok(compare_images(original_img, compared_img))
}

#[cfg(test)]
mod tests {
  use super::*;

    #[test]
    fn basic_test() {
        assert_eq!(1, 1);
    }
}
