use image::{open, DynamicImage};
use image_compare::*;

#[test]
fn when_the_images_are_equal_return_empty() {
    image_compare(
        "./tests/empty.png",
        compare_images_from_path("./tests/square.png", "./tests/square.png"),
    );
}

#[test]
fn when_the_picture_is_redemensioned_return_difference() {
    image_compare(
        "./tests/difference_redemensioned.png",
        compare_images_from_path("./tests/square.png", "./tests/redemensioned.png"),
    );
}

fn image_compare(original: &str, compared: DynamicImage) {
    let original_img = open(original).expect(&format!("Failed to open file {}", original));

    assert_eq!(original_img, compared);
}
