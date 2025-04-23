pub mod style;

use crate::node::style::NodeStyle;
use crate::utils::pos2::Pos2;
use crate::utils::size::Size;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Deserialize, Default, Serialize)]
pub enum Direction {
    #[default]
    Right,
    Left,
}

#[derive(Debug, Clone, Default, PartialEq, Deserialize, Serialize)]
pub struct Node {
    pub text: Option<String>,
    pub style_custom: Option<NodeStyle>,
    pub children: Option<Vec<Node>>,
    pub position_from_initial: Option<Pos2>,
    pub position_real: Option<Pos2>,
}

impl Node {
    pub fn new() -> Self {
        Self {
            text: None,
            children: None,
            style_custom: Option::from(NodeStyle::default()),
            position_from_initial: None,
            position_real: None,
        }
    }

    pub fn with_text(&mut self, text: String) -> Self {
        self.text = Some(text);
        self.clone()
    }

    pub fn with_children(&mut self, children: Vec<Node>) -> Self {
        self.children = Some(children);
        self.clone()
    }

    pub fn with_position(&mut self, position: Pos2) -> Self {
        self.position_from_initial = Some(position);
        self.clone()
    }

    pub fn get_children(&self) -> Option<Vec<Node>> {
        self.children.clone()
    }

    pub fn get_graphical_text_size(&self) -> Size {
        let NodeStyle {
            max_width,
            text_wrapping,
            font_size,
            ..
        } = self
            .style_custom
            .clone()
            .unwrap_or_else(|| NodeStyle::default());

        let font_char_width = font_size - 4.0;
        let font_char_height = font_size - 2.0;
        let font_interline = 4.0;

        let text_length = self.text.as_ref().map_or(0, |text| text.len()) as f32;
        let text_width = text_length * font_char_width;
        let text_height = if text_wrapping {
            let lines_count = (text_width / max_width).ceil();
            lines_count * font_char_height + (font_interline * lines_count)
        } else {
            font_char_height + font_interline
        };

        Size {
            width: text_width,
            height: text_height,
        }
    }

    pub fn get_graphical_size(&self) -> Size {
        // Calculate the size of the node based on its content
        let NodeStyle {
            min_width,
            max_width,
            min_height,
            text_wrapping,
            padding,
            ..
        } = self
            .style_custom
            .clone()
            .unwrap_or_else(|| NodeStyle::default());

        let Size {
            height: text_height,
            width: text_width,
        } = self.get_graphical_text_size();

        let new_width = match text_width + (padding * 2.0) {
            size if size < min_width => min_width,
            size if size > max_width && text_wrapping => max_width,
            _ => text_width + (padding * 2.0),
        };

        let new_height = match text_height + (padding * 2.0) {
            size if size < min_height => min_height,
            _ => text_height + (padding * 2.0),
        };

        let new_size = Size {
            width: new_width,
            height: new_height,
        };

        new_size
    }
    pub fn get_position_real(position_initial: Option<Pos2>, offset: Pos2) -> Option<Pos2> {
        match position_initial.clone() {
            None => None,
            Some(pos) => {
                tracing::trace!("get_position_real - pos: {:?} - offset: {:?}", pos, offset);
                Option::from(pos.subtract(&offset))
            }
        }
    }

    pub fn with_position_real(&mut self, offset: &Pos2) -> Self {
        match self.position_from_initial.clone() {
            None => self.position_real = Some(Pos2::default()),
            Some(pos) => {
                tracing::trace!("get_position_real - pos: {:?} - offset: {:?}", pos, offset);
                self.position_real = Some(pos.subtract(&offset));
            }
        }
        self.clone()
    }
}
