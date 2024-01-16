use image::open;
use image::DynamicImage;
use image::GenericImageView;
use image::ImageBuffer;
use image::Rgba;

fn compare_images(original: DynamicImage, compared: DynamicImage) -> DynamicImage {
    if original == compared {
        let (width, height) = original.dimensions();
        let response_image =
            ImageBuffer::from_fn(width, height, |_x, _y| Rgba([255, 255, 255, 255]));
        return DynamicImage::ImageRgba8(response_image);
    }

    DynamicImage::default()
}

pub fn compare_images_from_path(original: &str, compared: &str) -> DynamicImage {
    let original_img = open(original).expect(&format!("Failed to open file {}", original));
    let compared_img = open(compared).expect(&format!("Failed to open file {}", compared));
    compare_images(original_img, compared_img)
}

#[cfg(test)]
mod tests {

    #[test]
    fn basic_test() {
        assert_eq!(1, 1);
    }
}
