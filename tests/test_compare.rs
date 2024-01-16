use image::{open, DynamicImage};
use image_compare::*;

#[test]
fn basic_test() {
    assert_eq!(1, 1);
}

fn image_compare(original: &str, compared: &str) {
    let original_img = open(original).expect(&format!("Failed to open file {}", original));
    let compared_img = open(compared).expect(&format!("Failed to open file {}", compared));

    assert_eq!(original_img, compared_img);
}
