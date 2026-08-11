#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use super::*;
use eframe::egui;
use egui::ColorImage;
use image::DynamicImage;
use log::warn;
use std::path::PathBuf;

#[derive(Default)]
pub struct App {
    pub img1_path: String,
    pub img1: DynamicImage,
    pub img2_path: String,
    pub img2: DynamicImage,
}

impl eframe::App for App {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        let width = ui.available_width();
        let height = ui.available_height();

        place_image(
            &format!("original: {}", self.img1_path),
            &self.img1,
            egui::Pos2::new(0.0, 0.0),
            width * 0.5,
            height * 0.5,
            ui,
        );
        place_image(
            &format!("compared: {}", self.img2_path),
            &self.img2,
            egui::Pos2::new(width * 0.5, 0.0),
            width * 0.5,
            height * 0.5,
            ui,
        );
        place_image(
            "result",
            &compare_images(&self.img1, &self.img2),
            egui::Pos2::new(0.0, height * 0.5),
            width,
            height * 0.5,
            ui,
        );
    }
}

fn place_image(
    label: &str,
    img: &DynamicImage,
    pos: egui::Pos2,
    width: f32,
    height: f32,
    ui: &mut egui::Ui,
) -> egui::Response {
    let ctx = ui.ctx().clone();
    ui.allocate_rect(
        egui::Rect::from_min_size(pos, egui::vec2(width, height)),
        egui::Sense::hover(),
    );
    ui.allocate_ui_with_layout(
        egui::Vec2::new(width, height),
        egui::Layout::top_down_justified(egui::Align::Center),
        |ui| {
            ui.label(label);
            show_dynamic_image(&img, width, height * 0.9, ui, &ctx);
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
        Ok(color_img) => {
            let texture = ctx.load_texture(
                "result",
                color_img,
                egui::TextureOptions::default(),
            );
            ui.add(
                egui::Image::from_texture(&texture)
                    .max_width(width)
                    .max_height(height),
            )
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

pub fn path_to_label(file_name: &std::path::PathBuf) -> String {
    file_name
        .canonicalize()
        .expect("Unable to get the path")
        .to_str()
        .expect("Path not convertable")
        .to_string()
}

pub fn read_image_from_path(path: &PathBuf) -> DynamicImage {
    match image::open(path) {
        Ok(img) => img,
        Err(e) => {
            warn!("Couldn't open image {} ({})", path.display(), e);
            image::RgbImage::new(0, 0).into()
        }
    }
}
