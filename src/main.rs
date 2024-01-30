#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use eframe::egui;
use clap::Parser;
use std::path::PathBuf;

#[derive(Default)]
struct App {}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
          ui.columns(3, |columns| {
            columns[0].image("file://./tests/redemensioned.png");
            columns[1].image("file://./tests/square.png");
            columns[2].image("file://./tests/difference_redemensioned.png");
          });
        });
    }
}

#[derive(Parser)]
struct Args {
  img1: PathBuf,
  img2: PathBuf,
}


fn main() -> Result<(), eframe::Error> {
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
            Box::<App>::default()
        }),
    )
}

