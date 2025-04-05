mod text_utils;
mod nodes;

use crate::nodes::node::{draw_node, Node};
use eframe::egui;
use egui::{CentralPanel, ViewportBuilder};

fn main() {
    let options = eframe::NativeOptions {
        viewport: ViewportBuilder::default()
            .with_position([40000.0, 100.0])
            .with_inner_size([800.0, 600.0]),
        ..Default::default()
    };

    let text = "Hello, worldddddddddddddddddddddd!";

    eframe::run_simple_native("my egui app", options, move |ctx, _frame| {
        // let text_size = Vec2::new(150.0, 10.0);//measure_text(&ctx, text, egui::TextStyle::Heading);


        CentralPanel::default().show(ctx, |ui| {
            draw_node(ctx, ui, Node {
                text: text.to_string(),
            });
        });
    }).expect("TODO: panic message");
}


