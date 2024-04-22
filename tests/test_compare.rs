use image::{open, DynamicImage};
use ks_image_compare::*;

#[test]
fn when_the_images_are_equal_return_empty() -> Result<(), Error> {
    Ok(image_compare(
        "./tests/empty.png",
        compare_images_from_path("./tests/square.png", "./tests/square.png")?,
    ))
}

#[test]
fn when_the_picture_is_redemensioned_return_difference() -> Result<(), Error> {
    Ok(image_compare(
        "./tests/difference_redemensioned.png",
        compare_images_from_path("./tests/square.png", "./tests/redemensioned.png")?,
    ))
}

#[test]
fn images_with_different_size() -> Result<(), Error> {
    Ok(image_compare(
        "./tests/difference_ferris_diffent_image_sizes.png",
        compare_images_from_path(
            "./tests/cuddlyferris.png",
            "./tests/rustacean-flat-noshadow.png",
        )?,
    ))
}

fn image_compare(original: &str, compared: DynamicImage) {
    let original_img = open(original).expect(&format!("Failed to open file {}", original));

    assert_eq!(original_img, compared);
}
