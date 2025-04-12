use crate::mindmap::metadata::MindmapMetadata;
use crate::node::Node;
use serde::Deserialize;
use crate::mindmap::r#type::MindmapType;
use crate::mindmap::style::MindmapStyle;
use crate::utils::pos2::Pos2;
use crate::utils::size::Size;

pub mod metadata;
pub mod style;
pub mod r#type;

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct MindMap {
    pub metadata: Option<MindmapMetadata>,
    pub data: Option<Node>,
}

impl MindMap {
    pub fn new(metadata: Option<MindmapMetadata>, data: Option<Node>) -> Self {
        Self { metadata, data }
    }

    pub fn with_metadata(&self, metadata: MindmapMetadata) -> Self {
        Self { metadata: Some(metadata), ..self.clone() }
    }

    pub fn with_data(&self, data: Node) -> Self {
        Self { data: Some(data), ..self.clone() }
    }

    pub fn layout_mindmap(&mut self) {
        // Launch the layout process based on the diagram type
        match self.metadata.clone() {
            Some(metadata) => match metadata.diagram_type {
                Some(MindmapType::Standard) => self.layout_mindmap_standard(),
                None => self.layout_mindmap_standard()
            },
            None => return
        };
    }

    pub fn layout_mindmap_standard(&mut self) {


        let graphical_size = match self.data.clone() {
            Some(data) => data.get_graphical_size(),
            None => Size::default()
        };

        let data = match self.data.as_mut() {
            Some(data) => data,
            None => return
        };

        let children = match data.children.as_mut() {
            Some(children) => children,
            None => return
        };

        let MindmapStyle {
            padding_horizontal,
            padding_vertical
        } = match self.metadata.clone() {
            Some(metadata) => metadata.style.unwrap_or_else(|| MindmapStyle::default()),
            None => MindmapStyle::default()
        };

        tracing::debug!("Mindmap layout: padding_horizontal: {}, padding_vertical: {}", padding_horizontal, padding_vertical);

        // divide the children into two trees
        let mut right_tree: Vec<&mut Node> = Vec::new();
        let mut left_tree: Vec<&mut Node> = Vec::new();
        for (index, child) in children.iter_mut().enumerate() {
            match index {
                index if index % 2 == 0 => right_tree.push(child),
                _ => left_tree.push(child),
            }
        };

        let position_starting = match self.metadata.clone() {
            Some(metadata) => metadata.position_starting.unwrap_or_else(|| Pos2::new(500.0, 500.0)),
            None => Pos2::new(500.0, 500.0)
        };

        fn layout_mindmap_standard_children(
            current_tree: Vec<&mut Node>,
            parent_position: Pos2,
            parent_size: Size,
            side: f32, // +1.0 (droite), -1.0 (gauche)
            padding_horizontal: f32,
            padding_vertical: f32,
        ) -> f32 {

            let mut y_cursor = parent_position.y;
            let mut total_height = 0.0;
            let mut count = 0;
            for node in current_tree {

                let size = node.get_graphical_size();

                tracing::debug!("node: {:?}, - parent_position: {:?}, parent_size: {:?}, size: {:?}", node, parent_position, parent_size, size);

                // move to the right or left of the parent node
                node.position = Some(Pos2 {
                    x: parent_position.x + side * (parent_size.width / 2.0 + padding_horizontal + size.width / 2.0),
                    y: y_cursor,
                });

                // Layout récursif des enfants du node
                if let Some(children) = node.children.as_mut() {
                    if !children.is_empty() {
                        let subtree = children.iter_mut().collect::<Vec<&mut Node>>();
                        layout_mindmap_standard_children(
                            subtree,
                            node.position.clone().unwrap(),
                            size.clone(),
                            side,
                            padding_horizontal,
                            padding_vertical,
                        );
                    }
                }

                y_cursor += size.height + padding_vertical;
                total_height += size.height + padding_vertical;
                count += 1;
            }

            if count > 0 {
                total_height - padding_vertical
            } else {
                0.0
            }
        }

        // Layout gauche et droite
        let right_height = layout_mindmap_standard_children(
            right_tree,
            position_starting.clone(),
            graphical_size.clone(),
            1.0,
            padding_horizontal,
            padding_vertical,
        );

        let left_height = layout_mindmap_standard_children(
            left_tree,
            position_starting.clone(),
            graphical_size.clone(),
            -1.0,
            padding_horizontal,
            padding_vertical,
        );

        let total_height = right_height.max(left_height);

        // Center parent node on children
        self.data.as_mut().unwrap().position = Some(Pos2 {
            x: position_starting.x,
            y: position_starting.y + total_height / 2.0 - graphical_size.height / 2.0,
        });

    }

}

impl Default for MindMap {
    fn default() -> Self {
        Self {
            metadata: Some(MindmapMetadata::default()),
            data: Some(Node::default()),
        }
    }
}