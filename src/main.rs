#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use clap::Parser;
use eframe::egui;
use egui::ColorImage;
use image::DynamicImage;
use image_compare::*;
use std::path::PathBuf;

#[derive(Default)]
struct App {
    img1: DynamicImage,
    img2: DynamicImage,
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.columns(3, |columns| {
                show_dynamic_image(&self.img1, &mut columns[0], ctx);
                show_dynamic_image(&self.img2, &mut columns[1], ctx);

                show_dynamic_image(
                    &compare_images(&self.img1, &self.img2),
                    &mut columns[2],
                    ctx,
                );
            });
        });
    }
}

fn show_dynamic_image(
    img: &DynamicImage,
    ui: &mut egui::Ui,
    ctx: &egui::Context,
) -> egui::Response {
    match dynamic_image_to_egui(img) {
        Ok(img) => ui.image(&ctx.load_texture("result", img, egui::TextureOptions::default())),
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

#[derive(Parser)]
struct Args {
    img1: PathBuf,
    img2: PathBuf,
}

fn main() -> Result<(), anyhow::Error> {
    let args = Args::parse();
    let app = App {
        img1: image::open(args.img1)?,
        img2: image::open(args.img2)?,
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
    .expect("Something went wrong with egui");

    Ok(())
}
