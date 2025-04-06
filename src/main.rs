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

            let nodes = [
                (0, "Hello, world!", Pos2::new(140.0, 120.0), false),
                (1, "Hello, world 1!", Pos2::new(160.0, 250.0), false),
                (2, "Hello, world 2!", Pos2::new(360.0, 150.0), false),
                (3, "Hello, world back!", Pos2::new(340.0, 250.0), true),
                (4, "Hello, world back back!", Pos2::new(160.0, 480.0), false),
                (5, "Hello, world back!", Pos2::new(80.0, 550.0), false),
                (6, "Hello, world back back!", Pos2::new(350.0, 680.0), false)
            ];

            let node_rec_list = nodes.into_iter().map(|(id, text, pos, hidden)| {
                Node::new()
                    .with_text(id.to_string() + " - " + text.to_string().as_str())
                    .with_position(pos)
                    .with_id(id)
                    .hidden(hidden)
            }).collect::<Vec<_>>();

            let links = [
                (3, 0),
                (3, 1),
                (3, 2),
                (3, 4),
                (3, 5),
                (3, 6)
            ];

            let link_list = links.into_iter().map(|(source, target)| {
                let node_rec_source = Node::get_rect(ctx, node_rec_list[source].clone());
                let node_rec_target = Node::get_rect(ctx, node_rec_list[target].clone());
                Link::new_from_nodes(node_rec_source, node_rec_target)
            }).collect::<Vec<_>>();

            for link in link_list {
                link.draw_bezier(ui);
            }
            for node in node_rec_list {
                node.draw(ui);
            }

        });
    }).expect("TODO: panic message");
}


