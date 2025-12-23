use crate::app::*;
use ks_image_compare::*;

fn main() -> Result<(), anyhow::Error> {
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).
    let args = options::Args::parse();
    let app = App {
        img1_path: path_to_label(&args.img1),
        img1: read_image_from_path(&args.img1),
        img2_path: path_to_label(&args.img2),
        img2: read_image_from_path(&args.img2),
    };

    match args.output {
        Some(path) => {
            use std::io::{Cursor, Write};

            let out = compare_images(&app.img1, &app.img2);

            if path == "-" {
                let mut image_buffer = Vec::new();
                let mut cursor = Cursor::new(&mut image_buffer);
                out.write_to(&mut cursor, image::ImageOutputFormat::Png)?;

                let stdout = std::io::stdout();
                let mut handle = stdout.lock();
                handle.write_all(&image_buffer)?;
                handle.flush()?;
            } else {
                out.save(path).expect("could not save")
            }
        }
        None => {
            let options = eframe::NativeOptions {
                viewport: egui::ViewportBuilder::default().with_inner_size([600.0, 800.0]),
                ..Default::default()
            };
            eframe::run_native(
                "Compare Image",
                options,
                Box::new(|cc| {
                    // This gives us image support:
                    egui_extras::install_image_loaders(&cc.egui_ctx);
                    Ok(Box::new(app))
                }),
            )
            .expect("Something went wrong with egui");
        }
    }

    Ok(())
}
