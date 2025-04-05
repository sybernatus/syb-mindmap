mod text_utils;
mod nodes;
mod links;

use crate::nodes::node::Node;
use eframe::egui;
use eframe::emath::Pos2;
use eframe::epaint::{Color32, Stroke};
use egui::{CentralPanel, ViewportBuilder};
use crate::links::link::Link;

fn main() {
    let options = eframe::NativeOptions {
        viewport: ViewportBuilder::default()
            .with_position([40000.0, 100.0])
            .with_inner_size([480.0, 300.0]),
        ..Default::default()
    };

    eframe::run_simple_native("my egui app", options, move |ctx, _frame| {

        CentralPanel::default().show(ctx, |ui| {

            let node_1 = Node::new_with_text(
                "Hello, world!".to_string(),
                Pos2::new(140.0, 120.0)
            );
            let node_rec_1 = Node::get_rect(ctx, node_1.clone());

            let node_2 = Node::new_with_text(
                "Hello, world 2!".to_string(),
                Pos2::new(210.0, 210.0)
            );
            let node_rec_2 = Node::get_rect(ctx, node_2.clone());

            let link_1 = Link::new_from_nodes(node_rec_1, node_rec_2);

            link_1.draw_bezier(ui);
            node_1.draw(ui);
            node_2.draw(ui);
        });
    }).expect("TODO: panic message");
}


