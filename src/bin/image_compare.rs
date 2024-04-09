#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use eframe::egui;
use egui::ColorImage;
use image::DynamicImage;
use ks_image_compare::*;
use log::warn;
use std::path::PathBuf;

#[derive(Default)]
struct App {
    img1_path: String,
    img1: DynamicImage,
    img2_path: String,
    img2: DynamicImage,
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |mut ui| {
            let width = ui.available_width();
            let height = ui.available_height();

            place_image(
                &format!("original: {}", self.img1_path),
                &self.img1,
                egui::Pos2::new(0.0, 0.0),
                width * 0.5,
                height * 0.5,
                &mut ui,
                ctx,
            );
            place_image(
                &format!("compared: {}", self.img2_path),
                &self.img2,
                egui::Pos2::new(width * 0.5, 0.0),
                width * 0.5,
                height * 0.5,
                &mut ui,
                ctx,
            );
            place_image(
                "result",
                &compare_images(&self.img1, &self.img2),
                egui::Pos2::new(0.0, height * 0.5),
                width,
                height * 0.5,
                &mut ui,
                ctx,
            );
        });
    }
}

fn place_image(
    label: &str,
    img: &DynamicImage,
    pos: egui::Pos2,
    width: f32,
    height: f32,
    ui: &mut egui::Ui,
    ctx: &egui::Context,
) -> egui::Response {
    ui.allocate_ui_at_rect(
        egui::Rect::from_min_size(pos, egui::vec2(width, height)),
        |mut ui| {
            ui.label(label);
            show_dynamic_image(&img, width, height * 0.9, &mut ui, ctx);
        },
    )
    .response
}

fn show_dynamic_image(
    img: &DynamicImage,
    width: f32,
    height: f32,
    ui: &mut egui::Ui,
    ctx: &egui::Context,
) -> egui::Response {
    match dynamic_image_to_egui(img) {
        Ok(img) => {
            ui.allocate_ui_with_layout(
                egui::Vec2::new(width, height),
                egui::Layout::top_down_justified(egui::Align::Center),
                |ui| {
                    ui.add(
                        egui::Image::new(&ctx.load_texture(
                            "result",
                            img,
                            egui::TextureOptions::default(),
                        ))
                        .max_width(width)
                        .max_height(height),
                    )
                },
            )
            .response
        }
        Err(e) => ui.label(e),
    }
}

fn dynamic_image_to_egui(img: &DynamicImage) -> Result<egui::ColorImage, String> {
    let img_data = img.to_rgba8();

    Ok(ColorImage::from_rgba_unmultiplied(
        [
            img.width().try_into().or(Err("Problem with sizing"))?,
            img.height().try_into().or(Err("Problem with the height"))?,
        ],
        &img_data,
    ))
}

fn path_to_label(file_name: &std::path::PathBuf) -> String {
    file_name
        .canonicalize()
        .expect("Unable to get the path")
        .to_str()
        .expect("Path not convertable")
        .to_string()
}

fn read_image_from_path(path: &PathBuf) -> DynamicImage {
  match image::open(path) {
    Ok(img) => img,
    Err(e) => {
      warn!("Couldn't open image {} ({})", path.display(), e);
      image::RgbImage::new(0, 0).into()
    }
  }
}

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
