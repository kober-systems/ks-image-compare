use std::io::{Cursor, Write};

use ks_image_compare::*;

fn main() -> Result<(), anyhow::Error> {
    let args = options::Args::parse();

    let out = compare_images_from_path(args.img1.to_str().unwrap(), args.img2.to_str().unwrap())?;

    match args.output {
        Some(path) => out.save(path).expect("could not save"),
        None => {
            let mut image_buffer = Vec::new();
            let mut cursor = Cursor::new(&mut image_buffer);
            out.write_to(&mut cursor, image::ImageOutputFormat::Png)?;

            let stdout = std::io::stdout();
            let mut handle = stdout.lock();
            handle.write_all(&image_buffer)?;
            handle.flush()?;
        }
    }

    Ok(())
}
