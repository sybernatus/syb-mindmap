use std::default::Default;
use std::rc::Weak;
use crate::nodes::content::NodeContent;
use crate::text_utils::text_size::calculate_text_size;
use eframe::egui;
use eframe::epaint::StrokeKind;
use egui::{Color32, Context, Pos2, Rect, Vec2};

#[derive(Debug, Clone)]
pub struct NodeStyle {
    pub corner_radius: f32,
    pub background_color: Color32,
    pub border_color: Color32,
    pub hidden: bool,
    pub padding_horizontal: f32,
    pub padding_vertical: f32,
}

impl Default for NodeStyle {
    fn default() -> Self {
        Self {
            corner_radius: 15.0,
            background_color: Color32::from_rgb(122, 10, 0),
            border_color: Color32::from_rgb(255, 255, 255),
            hidden: false,
            padding_horizontal: 30.0,
            padding_vertical: 8.0,
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct Node {
    pub id: i32,
    pub content: NodeContent,
    pub position: Pos2,
    pub style: NodeStyle,
    pub parent: Option<Weak<Node>>,
}

impl Node {

    pub fn new() -> Self {
        Self {
            content: NodeContent::default(),
            position: Pos2::new(0.0, 0.0),
            style: NodeStyle::default(),
            id: rand::random::<i32>()
        }
    }

    pub fn with_id(&mut self, id: i32) -> Self {
        self.id = id;
        self.clone()
    }

    pub fn with_text(&mut self, text: String) -> Self {
        self.content.text = text;
        self.clone()
    }

    pub fn with_content(&mut self, content: NodeContent) -> Self {
        self.content = content;
        self.clone()
    }

    pub fn with_position(&mut self, position: Pos2) -> Self {
        self.position = position;
        self.clone()
    }
    pub fn with_style(&mut self, style: NodeStyle) -> Self {
        self.style = style;
        self.clone()
    }

    pub fn set_parent(&mut self, parent: Weak<Node>) -> Self {
        self.parent = Some(parent);
        self.clone()
    }

    pub fn hidden(&mut self, hidden: bool) -> Self {
        self.style.hidden = hidden;
        self.clone()
    }

    pub fn get_rect(ctx: &Context, node: Node) -> Rect {

        let Node { content, position, style, .. } = node.clone();
        let NodeContent { text, font_id, color } = content.clone();
        let text_size = calculate_text_size(ctx, text.as_str(), font_id.to_owned(), color);

        let node_size = Vec2::new(
            text_size.x + style.padding_horizontal * 2.0,
            text_size.y + style.padding_vertical * 2.0
        );

        Rect::from_center_size(position, node_size)
    }

    pub fn draw(&self, ui: &egui::Ui) {

        if self.style.hidden {
            return;
        }

        let node_shape = Self::get_rect(ui.ctx(), self.clone());
        ui.painter()
            .rect_filled(
                node_shape,
                self.style.corner_radius,
                self.style.background_color,
            );
        ui.painter()
            .rect_stroke(node_shape, self.style.corner_radius, egui::Stroke::new(1.0, self.style.border_color), StrokeKind::Inside);

        let NodeContent { text, font_id, color } = self.content.clone();
        let text_pos = Pos2::new(
            self.position.x,
            self.position.y,
        );
        ui.painter()
            .text(
                text_pos,
                egui::Align2::CENTER_CENTER,
                text,
                font_id,
                color,
            );
    }
}