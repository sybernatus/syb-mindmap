mod text_utils;
mod nodes;
mod links;

use crate::links::link::Link;
use crate::nodes::node::Node;
use eframe::{egui, NativeOptions};
use eframe::emath::Pos2;
use egui::{CentralPanel, ViewportBuilder};

fn main() {
    let options = eframe::NativeOptions {
        viewport: ViewportBuilder::default()
            .with_position([400.0, 100.0])
            .with_inner_size([480.0, 900.0]),
        ..NativeOptions::default()
    };

    eframe::run_simple_native("my egui app", options, move |ctx, _frame| {

        CentralPanel::default().show(ctx, |ui| {

            let node_1 = Node::new_with_text(
                "Hello, world!".to_string(),
                Pos2::new(140.0, 120.0)
            );

            let node_2 = Node::new_with_text(
                "Hello, world 2!".to_string(),
                Pos2::new(360.0, 150.0)
            );

            let node_rec_1 = Node::get_rect(ctx, node_1.clone());
            let node_rec_2 = Node::get_rect(ctx, node_2.clone());
            let link_1 = Link::new_from_nodes(node_rec_1, node_rec_2);


            let node_3 = Node::new_with_text(
                "Hello, world back!".to_string(),
                Pos2::new(340.0, 250.0)
            );

            let node_4 = Node::new_with_text(
                "Hello, world back back!".to_string(),
                Pos2::new(160.0, 480.0)
            );

            let node_rec_3 = Node::get_rect(ctx, node_3.clone());
            let node_rec_4 = Node::get_rect(ctx, node_4.clone());
            let link_2 = Link::new_from_nodes(node_rec_3, node_rec_4);



            let node_5 = Node::new_with_text(
                "Hello, world back!".to_string(),
                Pos2::new(80.0, 550.0)
            );

            let node_6 = Node::new_with_text(
                "Hello, world back back!".to_string(),
                Pos2::new(350.0, 680.0)
            );

            let node_rec_5 = Node::get_rect(ctx, node_5.clone());
            let node_rec_6 = Node::get_rect(ctx, node_6.clone());
            let link_2 = Link::new_from_nodes(node_rec_5, node_rec_6);
            let link_3 = Link::new_from_nodes(node_rec_1, node_rec_6);
            let link_4 = Link::new_from_nodes(node_rec_2, node_rec_5);


            link_2.draw_bezier(ui);
            link_3.draw_bezier(ui);
            link_2.draw_bezier(ui);
            link_4.draw_bezier(ui);
            link_1.draw_bezier(ui);
            node_1.draw(ui);
            node_2.draw(ui);
            node_3.draw(ui);
            node_4.draw(ui);
            node_5.draw(ui);
            node_6.draw(ui);

        });
    }).expect("TODO: panic message");
}


