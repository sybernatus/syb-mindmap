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
            (index as f32) * (100.0 / (total_brother as f32))
        } else {
            0.0
        };

        let position = Pos2::new(
            parent_position.x + 100.0 * (id_counter.borrow().rem_euclid(10) as f32),
            parent_position.y + vertical_offset * (id_counter.borrow().rem_euclid(10) as f32),
        );

        let node = Node {
            id: current_id,
            content: NodeContent {
                text: self.text.clone(),
                image: None,
            },
            position: position.clone(),
            style_custom: NodeStyleCustom::default(),
            parent_id,
        };

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


