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
    pub position_direction: Option<Direction>,
    pub children: Option<Vec<Node>>,
    pub position: Option<Pos2>,
}

impl Node {
    pub fn new() -> Self {
        Self {
            text: None,
            children: None,
            position_direction: Option::from(Direction::default()),
            style_custom: Option::from(NodeStyle::default()),
            position: None,
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
        self.position = Some(position);
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

    pub fn layout_mindmap_center(&mut self) -> Node {
        const H_PADDING: f32 = 150.0; // Horizontal padding between levels
        const V_PADDING: f32 = 20.0;

        self.position = Some(Pos2::new(0.0, 0.0));

        // Divide children into left and right based on their position direction
        let mut left_children = vec![];
        let mut right_children = vec![];

        let Size { width, height } = self.get_graphical_size();

        for child in self.children.as_mut().unwrap() {
            match child.position_direction {
                Some(Direction::Left) => left_children.push(child),
                _ => right_children.push(child),
            }
        }

        fn layout_children(
            mut side_children: Vec<&mut Node>,
            x_offset: f32,
            parent_node_position: Pos2,
            parent_node_size: Size,
        ) -> Vec<&mut Node> {
            let mut y_cursor = 0.0;
            for child in side_children.iter_mut() {
                let subtree_height = Node::layout_subtree(
                    child,
                    x_offset,
                    y_cursor,
                    parent_node_position.clone(),
                    parent_node_size.clone(),
                );
                y_cursor += subtree_height + V_PADDING;
            }
            side_children
        }

        let left_offset = H_PADDING;
        let right_offset = H_PADDING;
        let _left_children = layout_children(
            left_children,
            left_offset,
            self.position.clone().unwrap(),
            Size::from(Size { width, height }),
        );
        let _right_children = layout_children(
            right_children,
            right_offset,
            self.position.clone().unwrap(),
            Size::from(Size { width, height }),
        );

        self.layout_parent_position(self.position.clone().unwrap().x);
        self.to_owned()
    }

    fn layout_subtree(
        &mut self,
        _offset_x: f32,
        offset_y: f32,
        parent_position: Pos2,
        parent_size: Size,
    ) -> f32 {

        let Size { width, height } = self.get_graphical_size();

        let node_half_width = width / 2.0;
        let parent_half_width = parent_size.width / 2.0;

        let node_half_height = height / 2.0;
        let parent_half_height = parent_size.height / 2.0;
        let node_offset_y = offset_y + node_half_height + parent_half_height + parent_position.y;

        let node_offset_x = match self.position_direction {
            Some(Direction::Left) => {
                parent_position.x - _offset_x - node_half_width - parent_half_width
            }
            _ => parent_position.x + _offset_x + node_half_width + parent_half_width,
        };

        if self.children.as_mut().unwrap().is_empty() {
            self.position = Some(Pos2::new(node_offset_x, node_offset_y));
            return height;
        }

        let mut y_cursor = offset_y;
        let mut total_height = 0.0;

        for child in self.children.as_mut().unwrap() {
            child.position_direction = Some(self.position_direction.clone().unwrap_or_default());
            tracing::debug!(
                "child: {:?}, - node_offset_x: {:?}, y_cursor: {:?}, parent_position: {:?}, parent_size: {:?}",
                child,
                node_offset_x,
                y_cursor,
                Pos2::new(node_offset_x, node_offset_y),
                Size::from(Size { width, height })
            );
            let subtree_height = Self::layout_subtree(
                child,
                node_offset_x,
                y_cursor,
                Pos2::new(node_offset_x, node_offset_y),
                Size::from(Size { width, height }),
            );
            y_cursor += subtree_height + offset_y;
            total_height += subtree_height + offset_y;
        }

        self.layout_parent_position(node_offset_x);
        tracing::debug!(
            "text: {:?} - self.position.x: {:?} - self.position.y: {:?}",
            self.text,
            self.position.clone().unwrap().x,
            self.position.clone().unwrap().y
        );

        total_height.max(height)
    }

    fn layout_parent_position(&mut self, parent_position_x: f32) {
        let children = self.children.as_mut().unwrap();
        let fist_child = children.first().unwrap();
        let last_child = children.last().unwrap();

        let children_middle = children.len() / 2;

        let first_y = fist_child.clone().position.unwrap().y;
        let last_y = last_child.clone().position.unwrap().y;
        tracing::debug!(
            "text: {:?} - first_y: {:?} - last_y: {:?}",
            self.text,
            first_y,
            last_y
        );

        self.position = Some(Pos2::new(
            parent_position_x,
            children[children_middle].clone().position.unwrap().y,
        ));
    }

    pub fn get_node_bounding_box(&self) -> Option<(Pos2, Size)> {
        let mut min_x = f32::MAX;
        let mut min_y = f32::MAX;
        let mut max_x = f32::MIN;
        let mut max_y = f32::MIN;

        fn traverse(node: &Node, min_x: &mut f32, min_y: &mut f32, max_x: &mut f32, max_y: &mut f32) {
            if let (Some(pos), Some(size)) = (node.clone().position, Some(node.get_graphical_size())) {
                let half_w = size.width / 2.0;
                let half_h = size.height / 2.0;

                let left = pos.x - half_w;
                let right = pos.x + half_w;
                let top = pos.y - half_h;
                let bottom = pos.y + half_h;

                *min_x = min_x.min(left);
                *max_x = max_x.max(right);
                *min_y = min_y.min(top);
                *max_y = max_y.max(bottom);
            }

            if let Some(children) = &node.children {
                for child in children {
                    traverse(child, min_x, min_y, max_x, max_y);
                }
            }
        }

        traverse(self, &mut min_x, &mut min_y, &mut max_x, &mut max_y);

        if min_x == f32::MAX || min_y == f32::MAX {
            return None;
        }

        let width = max_x - min_x;
        let height = max_y - min_y;
        tracing::debug!(
            "get_node_bounding_box - min_x: {:?}, min_y: {:?}, width: {:?}, height: {:?}",
            min_x, min_y, width, height
        );
        Some((Pos2::new(min_x, min_y), Size { width, height }))
    }
}
