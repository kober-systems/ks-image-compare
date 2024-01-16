use image::open;
use image::DynamicImage;
use image::GenericImageView;
use image::ImageBuffer;
use image::Rgba;

fn compare_images(original: DynamicImage, compared: DynamicImage) -> DynamicImage {
    let (width, height) = original.dimensions();

    DynamicImage::ImageRgba8(ImageBuffer::from_fn(width, height, |x, y| {
        let original_pixel = original.get_pixel(x, y);
        let compared_pixel = compared.get_pixel(x, y);
        if original_pixel == compared_pixel {
            Rgba([255, 255, 255, 255])
        } else {
            Rgba([255, 0, 0, 255])
        }
    }))
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
