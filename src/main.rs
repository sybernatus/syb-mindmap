mod text_utils;
mod nodes;

use crate::nodes::content::NodeContent;
use crate::nodes::node::{Node};
use eframe::egui;
use eframe::emath::Pos2;
use eframe::epaint::{Color32, Stroke};
use egui::{CentralPanel, ViewportBuilder};

fn main() {
    let options = eframe::NativeOptions {
        viewport: ViewportBuilder::default()
            .with_position([40000.0, 100.0])
            .with_inner_size([480.0, 300.0]),
        ..Default::default()
    };

    eframe::run_simple_native("my egui app", options, move |ctx, _frame| {

        CentralPanel::default().show(ctx, |ui| {

            let node_1 = Node::new(Node {
                content: NodeContent {
                    text: "Hello, world!".to_string(),
                    font_id: egui::FontId::proportional(20.0),
                    color: Color32::from_rgb(255, 255, 255),
                    ..NodeContent::default()
                },
                position: Pos2::new(10.0, 20.0),
                ..Node::default()
            });
            let node_rec_1 = Node::get_rect(ctx, node_1.clone());

            let node_2 = Node::new( Node{
                content: NodeContent {
                    text: "Hello, world 2!".to_string(),
                    font_id: egui::FontId::proportional(20.0),
                    color: Color32::from_rgb(255, 255, 255),
                    ..NodeContent::default()
                },
                position: Pos2::new(10.0, 160.0),
                ..Node::default()
            });
            let node_rec_2 = Node::get_rect(ctx, node_2.clone());
            ui.painter().line_segment([node_rec_1.center(), node_rec_2.center()], Stroke::new(1.5, Color32::LIGHT_GRAY));
            node_1.draw(ui);
            node_2.draw(ui);
        });
    }).expect("TODO: panic message");
}


