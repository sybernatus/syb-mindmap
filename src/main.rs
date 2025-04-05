mod text_utils;
mod nodes;

use crate::nodes::content::NodeContent;
use crate::nodes::node::{node_create, node_draw, Node};
use eframe::egui;
use eframe::emath::Pos2;
use eframe::epaint::{Color32, Stroke};
use egui::{CentralPanel, ViewportBuilder};

fn main() {
    let options = eframe::NativeOptions {
        viewport: ViewportBuilder::default()
            .with_position([40000.0, 100.0])
            .with_inner_size([800.0, 600.0]),
        ..Default::default()
    };

    eframe::run_simple_native("my egui app", options, move |ctx, _frame| {

        CentralPanel::default().show(ctx, |ui| {

            let node_1 = Node {
                content: NodeContent {
                    text: "Hello, world!".to_string(),
                    font_id: egui::FontId::proportional(20.0),
                    color: Color32::from_rgb(255, 255, 255),
                    ..NodeContent::default()
                },
                position: Pos2::new(10.0, 20.0),
                ..Node::default()
            };
            let node_rec_1 = node_create(ctx, node_1.clone());

            let node_2 = Node {
                content: NodeContent {
                    text: "Hello, world 2!".to_string(),
                    font_id: egui::FontId::proportional(20.0),
                    color: Color32::from_rgb(255, 255, 255),
                    ..NodeContent::default()
                },
                position: Pos2::new(10.0, 160.0),
                ..Node::default()
            };
            let node_rec_2 = node_create(ctx, node_2.clone());
            ui.painter().line_segment([node_rec_1.center(), node_rec_2.center()], Stroke::new(1.5, Color32::LIGHT_GRAY));
            node_draw(ui, node_rec_1, node_1);
            node_draw(ui, node_rec_2, node_2);
        });
    }).expect("TODO: panic message");
}


