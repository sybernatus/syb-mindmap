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
    #[serde(default = "NodeStyle::new")]
    pub style_custom: NodeStyle,
    pub children: Option<Vec<Node>>,
    pub position_from_initial: Option<Pos2>,
    pub position_real: Option<Pos2>,
    pub parent: Option<Box<Node>>,
    pub graphical_size: Option<Size>,
    pub children_graphical_size: Option<Size>,
}

impl Node {
    pub fn new() -> Self {
        Self {
            text: None,
            children: None,
            style_custom: NodeStyle::default(),
            position_from_initial: None,
            position_real: None,
            parent: None,
            graphical_size: None,
            children_graphical_size: None,
        }
    }

    pub fn with_text(&mut self, text: String) -> &mut Node {
        self.text = Some(text);
        self
    }

    pub fn with_children(&mut self, children: Vec<Node>) -> &mut Node {
        self.children = Some(children);
        self
    }

    pub fn with_position(&mut self, position: Pos2) -> &mut Node {
        self.position_from_initial = Some(position);
        self
    }

    pub fn with_parent(&mut self, node: Self) -> &mut Node {
        self.parent = Some(Box::new(node));
        self
    }

    /// Get the children of the node.
    pub fn get_children(&self) -> Option<&Vec<Node>> {
        self.children.as_ref()
    }

    /// Get the graphical size of the node text depending on its style.
    pub fn get_graphical_text_size(&self) -> Size {
        let NodeStyle {
            max_width,
            text_wrapping,
            font_size,
            ..
        } = self
            .style_custom
            .clone();

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

    /// Set the graphical size of the node.
    pub fn compute_graphical_size(&mut self) -> &mut Node {
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
            .clone();

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
        self.graphical_size = Some(new_size.clone());
        tracing::debug!("compute_graphical_size - new_size: {:?}", new_size);
        self
    }


    /// Compute the vertical graphical size of the direct children subtree.
    pub fn compute_children_graphical_size(&mut self, padding_vertical: f32, padding_horizontal: f32) -> &mut Node {
        if let Some(children) = &self.children {
            let mut children_graph_size = Size::new(0.0, 0.0);
            for child in children {
                let child_children_size = child.clone().children_graphical_size.unwrap_or(Size::new(0.0, 0.0));
                let child_size = child.clone().graphical_size.unwrap_or(Size::new(0.0, 0.0));
                children_graph_size.height += child_children_size.height.max(child_size.height) + padding_vertical;
                children_graph_size.width = children_graph_size.width.max(child_size.width + child_children_size.width + padding_horizontal);
            }
            self.children_graphical_size = Some(children_graph_size);
        }
        self
    }



    /// Get the graphical size of the node.
    pub fn get_graphical_size(&self) -> Size {
        self.graphical_size
            .clone()
            .unwrap_or_else(|| Size::default())
    }

    /// Computes the real position of the node based on its initial position and the offset.
    pub fn with_position_real(&mut self, offset: &Pos2) -> &Self {
        match self.position_from_initial.clone() {
            None => self.position_real = Some(Pos2::default()),
            Some(pos) => {
                tracing::debug!("get_position_real - pos: {:?} - offset: {:?}", pos, offset);
                self.position_real = Some(pos.subtract(&offset));
            }
        }
        self
    }

    /// Returns true if the node is a root node.
    pub fn is_root(&self) -> bool {
        self.parent.is_none()
    }
}
