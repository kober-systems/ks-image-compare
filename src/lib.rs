use image::open;
use image::DynamicImage;

fn compare_images(original: DynamicImage, compared: DynamicImage) -> DynamicImage {
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
