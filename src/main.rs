#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use eframe::egui;
use clap::Parser;
use std::path::PathBuf;
use image_compare::*;
use image::DynamicImage;
use egui::ColorImage;

#[derive(Default)]
struct App {
  img1: String,
  img2: String,
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
          ui.columns(3, |columns| {
            columns[0].image(format!("file://{}",self.img1));
            columns[1].image(format!("file://{}",self.img2));

            match compare_images_from_path(&self.img1, &self.img2) {
              Ok(img) => columns[2].image(&ctx.load_texture(
                "result", dynamic_image_to_egui(img), egui::TextureOptions::default())),
              Err(_e) => columns[2].label("No image found"),
            }
          });
        });
    }
}

fn dynamic_image_to_egui(img: DynamicImage) -> egui::ColorImage {
  let img_data = img.to_rgba8();
  ColorImage::from_rgba_unmultiplied([
      img.width().try_into().unwrap(),
      img.height().try_into().unwrap()],
      &img_data)
}

#[derive(Parser)]
struct Args {
  img1: PathBuf,
  img2: PathBuf,
}

fn main() -> Result<(), eframe::Error> {
    let args = Args::parse();
    let app = App {
      img1: args.img1.to_str().unwrap().to_string(),
      img2: args.img2.to_str().unwrap().to_string(),
    };

    //env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([600.0, 800.0]),
        ..Default::default()
    };
    eframe::run_native(
        "Image Viewer",
        options,
        Box::new(|cc| {
            // This gives us image support:
            egui_extras::install_image_loaders(&cc.egui_ctx);
            Box::new(app)
        }),
    )
}

