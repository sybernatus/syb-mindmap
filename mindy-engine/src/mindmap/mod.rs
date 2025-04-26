use crate::mindmap::metadata::MindmapMetadata;
use crate::mindmap::style::MindmapStyle;
use crate::mindmap::r#type::MindmapType;
use crate::node::Node;
use crate::utils::pos2::Pos2;
use crate::utils::size::Size;
use serde::{Deserialize, Serialize};
use std::error::Error;

pub mod metadata;
pub mod style;
pub mod r#type;

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct Mindmap {
    #[serde(default)]
    pub metadata: MindmapMetadata,
    pub data: Option<Node>,
    pub size: Option<Size>,
    pub position: Option<Pos2>,
}

impl Mindmap {
    pub fn new(metadata: MindmapMetadata, data: Option<Node>) -> Self {
        Self { metadata, data, size: None, position: None }
    }

    pub fn with_metadata(&self, metadata: MindmapMetadata) -> Self {
        Self {
            metadata,
            ..self.clone()
        }
    }

    pub fn with_data(&self, data: Node) -> Self {
        Self {
            data: Some(data),
            ..self.clone()
        }
    }

    pub fn layout_mindmap(&mut self) -> &mut Self {
        // Launch the layout process based on the diagram type
        match self.metadata.diagram_type {
            MindmapType::Standard => self.layout_mindmap_standard(),
        }
    }

    /// Calculates the position of each nodes following the mindmap standard layout
    pub fn layout_mindmap_standard(&mut self) -> &mut Self {
        let graphical_size = match self.data.clone() {
            Some(data) => data.get_graphical_size(),
            None => Size::default(),
        };

        let data = match self.data.as_mut() {
            Some(data) => data,
            None => return self,
        };

        let children = match data.children.as_mut() {
            Some(children) => children,
            None => &mut vec![],
        };

        let MindmapStyle {
            padding_horizontal,
            padding_vertical,
            ..
        } = self.metadata.style;

        tracing::trace!(
            "Mindmap layout: padding_horizontal: {}, padding_vertical: {}",
            padding_horizontal,
            padding_vertical
        );

        // divide the children into two trees
        let mut right_tree: Vec<&mut Node> = Vec::new();
        let mut left_tree: Vec<&mut Node> = Vec::new();
        for (index, child) in children.iter_mut().enumerate() {
            match index {
                index if index % 2 == 0 => right_tree.push(child),
                _ => left_tree.push(child),
            }
        }

        let position_starting = Pos2::new(0.0, 0.0);

        fn layout_mindmap_standard_children(
            current_tree: Vec<&mut Node>,
            parent_position: Pos2,
            parent_size: Size,
            side: f32,
            padding_horizontal: f32,
            padding_vertical: f32,
        ) -> f32 {
            let mut y_cursor = parent_position.y;
            let mut total_height = 0.0;
            let mut count = 0;
            for node in current_tree {
                let size = node.get_graphical_size();
                y_cursor += size.height / 2.0 + padding_vertical;

                tracing::debug!(
                    "parent_position: {:?}, parent_size: {:?}, size: {:?}",
                    parent_position,
                    parent_size,
                    size
                );

                // Calculating the position of the node depending on the parent node
                node.position_from_initial = Some(Pos2 {
                    x: parent_position.x + side * (parent_size.width / 2.0 + padding_horizontal + size.width / 2.0),
                    y: y_cursor,
                });

                // Recursively layout the children of the node
                if let Some(children) = node.children.as_mut() {
                    if !children.is_empty() {
                        let subtree = children.iter_mut().collect::<Vec<&mut Node>>();
                        layout_mindmap_standard_children(
                            subtree,
                            node.position_from_initial.clone().unwrap(),
                            size.clone(),
                            side,
                            padding_horizontal,
                            padding_vertical,
                        );
                    }
                }

                y_cursor += size.height / 2.0 + padding_vertical;
                tracing::debug!(
                    "y_cursor: {:?}, size: {:?}, text: {:?}",
                    y_cursor,
                    size,
                    node.text.clone().unwrap_or_default()
                );
                total_height += size.height + padding_vertical;
                count += 1;
            }

            if count > 0 {
                total_height - padding_vertical
            } else {
                0.0
            }
        }

        // Layout right tree
        let right_height = layout_mindmap_standard_children(
            right_tree,
            position_starting.clone(),
            graphical_size.clone(),
            1.0,
            padding_horizontal,
            padding_vertical,
        );

        // Layout left tree
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
        self.data.as_mut().unwrap().position_from_initial = Some(Pos2 {
            x: position_starting.x,
            y: position_starting.y + total_height / 2.0 - graphical_size.height / 2.0,
        });

        self
    }


    /// Initializes the mindmap from a json string
    pub fn from_json_string(json_string: String) -> Result<Self, impl Error> {
        match serde_json::from_str::<Self>(json_string.as_str()) {
            Ok(mindmap) => {
                tracing::trace!("Mindmap from_json_string - {:?}", mindmap);
                let mindmap = Mindmap::new(mindmap.metadata, mindmap.data);
                Ok(mindmap)
            }
            Err(e) => {
                tracing::error!("Error decoding json: {:?}", e);
                Err(e)
            }
        }
    }

    /// Initializes the mindmap from a yaml string
    pub fn from_yaml_string(yaml_str: String) -> Result<Self, impl Error> {
        match serde_yaml::from_str::<Self>(yaml_str.as_str()) {
            Ok(mindmap) => {
                tracing::trace!("Mindmap from_yaml_str - {:?}", mindmap);
                let mindmap = Mindmap::new(mindmap.metadata, mindmap.data);
                Ok(mindmap)
            }
            Err(e) => {
                tracing::error!("Error decoding yaml: {:?}", e);
                Err(e)
            }
        }
    }

    /// Initializes the bounding box of the mindmap.
    /// Traverses all the node position and calculate the top left corner position & the width/height of the mindmap
    pub fn with_bounding_box(&mut self) -> &mut Self {
        let extra_offset = Pos2::new(10.0, 10.0);
        let mut min_x = f32::MAX;
        let mut min_y = f32::MAX;
        let mut max_x = f32::MIN;
        let mut max_y = f32::MIN;

        fn traverse(node: Node, min_x: &mut f32, min_y: &mut f32, max_x: &mut f32, max_y: &mut f32) {
            if let (Some(pos), Some(size)) = (node.clone().position_from_initial, Some(node.get_graphical_size())) {
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

            if let Some(children) = node.children {
                for child in children {
                    traverse(child, min_x, min_y, max_x, max_y);
                }
            }
        }

        traverse(self.clone().data.unwrap_or_default(), &mut min_x, &mut min_y, &mut max_x, &mut max_y);

        if min_x == f32::MAX || min_y == f32::MAX {
            return self;
        }

        let width = max_x - min_x;
        let height = max_y - min_y;
        tracing::trace!(
            "get_node_bounding_box - min_x: {:?}, min_y: {:?}, width: {:?}, height: {:?}",
            min_x, min_y, width, height
        );
        self.position = Some(Pos2::new(min_x, min_y).subtract(&extra_offset));
        self.size = Some(Size { width, height });
        self
    }

    /// Computes the real position of the mindmap nodes and all its children.
    /// The mindmap position is used as offset
    pub fn compute_real_position(&mut self) -> &mut Self {
        // moving through all nodes children and calculate with_position_real()
        let offset = self.clone().position.unwrap_or_default();
        if let Some(ref mut data) = self.data {
            fn traverse(node: &mut Node, offset: &Pos2) {
                if let Some(children) = node.children.as_mut() {
                    for child in children {
                        traverse(child, offset);
                    }
                }
                node.with_position_real(&offset);
            }
            data.with_position_real(&offset);
            traverse(data, &offset);
        }

        self
    }

    /// Assigns the parent node to all nodes in the mindmap.
    pub fn compute_parents(&mut self) -> &mut Self {
        fn assign_parents_recursively(node: &mut Node, parent: Option<Node>) {
            node.parent = parent.map(Box::new);

            if let Some(mut children) = node.children.take() {
                let node_clone = node.clone();

                for child in children.iter_mut() {
                    assign_parents_recursively(child, Some(node_clone.clone()));
                }

                node.children = Some(children);
            }
        }

        if let Some(root) = self.data.as_mut() {
            assign_parents_recursively(root, None);
        }
        self
    }

    /// Computes all the mindmap properties:
    /// - layout mindmap
    /// - layout bounding box
    /// - node real position
    /// - node parents
    /// - node style
    pub fn compute_all(&mut self) -> &Self {


        self.layout_mindmap()
            .with_bounding_box()
            .compute_real_position()
            .compute_parents();

        // Compute background color
        self.compute_node_style(
                |node: &Node| node.style_custom.background_color.clone(),
                |node: &mut Node, value| { node.style_custom.with_background_color(value); },
                self.metadata.global_node_style.background_color.clone().unwrap(),
                self.metadata.style.root_node_color.clone()
        );

        self
    }

    /// Computes node style based on the mindmap global style & the node style
    pub fn compute_node_style<T, FGet, FSet>(&mut self, mut get: FGet, mut set: FSet, default_value: T, parent_value: T) -> &Self
    where
        T: Clone,
        FGet: FnMut(&Node) -> Option<T>,
        FSet: FnMut(&mut Node, T),
    {
        if let Some(ref mut data) = self.data {
            fn traverse<T, FGet, FSet>(node: &mut Node, parent_value: T, get: &mut FGet, set: &mut FSet)
            where
                T: Clone,
                FGet: FnMut(&Node) -> Option<T>,
                FSet: FnMut(&mut Node, T),
            {
                let value = get(node).unwrap_or_else(|| parent_value.clone());
                set(node, value.clone());

                if let Some(children) = node.children.as_mut() {
                    for child in children {
                        traverse(child, value.clone(), get, set);
                    }
                }
            }

            traverse(data, default_value, &mut get, &mut set);

            set(data, parent_value);
        }

        self
    }
}

impl Default for Mindmap {
    fn default() -> Self {
        Self {
            metadata: MindmapMetadata::default(),
            data: Some(Node::default()),
            size: None,
            position: None,
        }
    }
}
