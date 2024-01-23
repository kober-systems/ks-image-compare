use image_compare::*;

fn main() {
    compare_images_from_path("./tests/v1.png", "./tests/v2.png").unwrap()
        .save("out.png")
        .expect("could not save");
}
