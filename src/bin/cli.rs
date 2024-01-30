use image_compare::*;

fn main() -> Result<(), anyhow::Error> {
    let args = options::Args::parse();

    compare_images_from_path(args.img1.to_str().unwrap(), args.img2.to_str().unwrap())?
        .save("out.png")
        .expect("could not save");

    Ok(())
}
