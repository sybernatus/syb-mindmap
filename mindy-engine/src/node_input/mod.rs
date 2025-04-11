use std::cell::RefCell;
use serde::Deserialize;
use crate::node::{Node, NodeContent, NodeStyleCustom, Pos2};

#[derive(Debug, Clone, Default, PartialEq, Deserialize)]
pub struct NodeInput {
    pub text: Option<String>,
    pub children: Option<Vec<NodeInput>>,
}

impl NodeInput {
    pub fn new() -> Self {
        Self {
            text: None,
            children: None,
        }
    }

    pub fn with_text(&mut self, text: String) -> Self {
        self.text = Some(text);
        self.clone()
    }

    pub fn with_children(&mut self, children: Vec<NodeInput>) -> Self {
        self.children = Some(children);
        self.clone()
    }

    pub fn to_node_vec(
        &self,
        parent_id: Option<i32>,
        id_counter: &RefCell<i32>,
        parent_position: Pos2,
        index: usize,
        total_brother: usize,
        out: &mut Vec<Node>,
    ) -> Vec<Node> {
        let current_id = {
            let mut id = id_counter.borrow_mut();
            let my_id = *id;
            *id += 1;
            my_id
        };

        // Calculate vertical position based on index and total brothers
        let vertical_offset = if total_brother > 1 {
            (index as f32) * (300.0 / (total_brother as f32))
        } else {
            0.0
        };

        // total child = 5
        // index = 0 1 2 3 4
        // vertical_offset = 0.0 20.0 40.0 60.0 80.0

        let position = Pos2::new(
            parent_position.x + 200.0,
            parent_position.y + vertical_offset,
        );

        // create node with new
        let node = Node::new()
            .with_text(self.text.clone().unwrap())
            .with_position(position.clone())
            .with_id(current_id)
            .set_parent(parent_id.unwrap_or(0))
            .with_style_custom(NodeStyleCustom {
                text_wrapping: true,
                ..NodeStyleCustom::default()
            });

        out.push(node);

        let total_brother = &self.children.clone().unwrap().len();
        if let Some(children) = &self.children {
            for (index, child) in children.iter().enumerate() {
                Self::to_node_vec(child, Some(current_id), id_counter, position.clone(), index, total_brother.clone(), out);
            }
        }

        out.to_owned()
    }

}


