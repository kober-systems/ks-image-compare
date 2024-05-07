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
            Box::new(app)
        }),
    )
    .expect("Something went wrong with egui");

    Ok(())
}
