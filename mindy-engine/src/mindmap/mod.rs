use crate::mindmap::metadata::MindmapMetadata;
use crate::mindmap::r#type::MindmapType;
use crate::node::Node;
use crate::layout::pos2::Pos2;
use crate::layout::size::Size;
use serde::{Deserialize, Serialize};
use std::error::Error;
use crate::layout::left_right_bottom::LeftRightBottomLayout;
use crate::layout::left_right_horizontal::LeftRightHorizontalLayout;
use crate::layout::Position2D;


pub mod metadata;
pub mod style;
pub mod r#type;

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(default)]
pub struct Mindmap {
    pub metadata: MindmapMetadata,
    pub data: Option<Node>,
    #[serde(skip)]
    pub position: Option<Pos2>,
    #[serde(skip)]
    pub size: Option<Size>,
}

impl Default for Mindmap {
    fn default() -> Self {
        Self {
            metadata: MindmapMetadata::default(),
            data: None,
            size: None,
            position: None,
        }
    }
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
            MindmapType::LeftRightHorizontal => LeftRightHorizontalLayout::layout(self),
            MindmapType::LeftRightBottom => LeftRightBottomLayout::layout(self),
        }
    }

    /// Computes the vertical graphical size of all the nodes in the mindmap
    pub fn compute_nodes_subtree_graphical_size(&mut self) -> &mut Self {

        let padding_vertical = self.metadata.style.padding_vertical;
        let padding_horizontal = self.metadata.style.padding_horizontal;

        fn traverse(node: &mut Node, padding_vertical: f32, padding_horizontal: f32) {
            if let Some(children) = node.children.as_mut() {
                for child in children {
                    traverse(child, padding_vertical, padding_horizontal);
                }
            }
            node.compute_children_graphical_size(padding_vertical, padding_horizontal);
        }

        if let Some(ref mut data) = self.data {
            traverse(data, padding_vertical, padding_horizontal);
        }

        self
    }


    /// Initializes the mindmap from a json string
    /// # Arguments
    /// * `json_string` - The json string to decode
    /// # Returns
    /// * `Result<Self, impl Error>` - The decoded mindmap
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
    /// # Arguments
    /// * `yaml_str` - The yaml string to decode
    /// # Returns
    /// * `Result<Self, impl Error>` - The decoded mindmap
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
        let padding_horizontal = self.metadata.style.padding_horizontal;
        let padding_vertical = self.metadata.style.padding_vertical;

        fn traverse(node: Node, min_x: &mut f32, min_y: &mut f32, max_x: &mut f32, max_y: &mut f32, padding_horizontal: f32, padding_vertical: f32) {
            if let (Some(pos), Some(size)) = (node.clone().position_from_initial, Some(node.get_graphical_size())) {
                let half_w = size.width / 2.0 + padding_horizontal;
                let half_h = size.height / 2.0 + padding_vertical;

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
                    traverse(child, min_x, min_y, max_x, max_y, padding_horizontal, padding_vertical);
                }
            }
        }

        traverse(self.clone().data.unwrap_or_default(), &mut min_x, &mut min_y, &mut max_x, &mut max_y, padding_horizontal, padding_vertical);

        if min_x == f32::MAX || min_y == f32::MAX {
            return self;
        }

        let width = max_x - min_x;
        let height = max_y - min_y;
        tracing::trace!(
            "get_node_bounding_box - min_x: {:?}, min_y: {:?}, width: {:?}, height: {:?}",
            min_x, min_y, width, height
        );
        self.position = Some(Pos2::new(min_x, min_y).subtract(&extra_offset).clone());
        self.size = Some(Size { width, height });
        self
    }

    /// Computes the real position of the mindmap nodes and all its children.
    /// The mindmap position is used as offset
    pub fn compute_real_position(&mut self) -> &mut Self {
        // moving through all nodes children and calculate with_position_real()
        let offset = self.clone().position.unwrap_or(Pos2::zero());
        if let Some(ref mut data) = self.data {
            fn traverse(node: &mut Node, offset: &Pos2) {
                if let Some(children) = node.children.as_mut() {
                    for child in children {
                        traverse(child, offset);
                    }
                }
                node.with_position_real(&offset);
            }
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

    /// Computes the nodes graphical size
    pub fn compute_graphical_size(&mut self) -> &mut Self {

        fn traverse(node: &mut Node) {
            node.compute_graphical_size();
            if let Some(children) = node.children.as_mut() {
                for child in children {
                    traverse(child);
                }
            }
        }

        if let Some(ref mut data) = self.data {
            traverse(data);
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

        self.compute_graphical_size()
            .compute_nodes_subtree_graphical_size()
            .layout_mindmap()
            .with_bounding_box()
            .compute_real_position()
            .compute_parents();

        // Compute background color
        self.compute_node_style(
            |node: &Node| Some(node.style_custom.background_color.clone()),
            |node: &mut Node, value| { node.style_custom.with_background_color(value); },
            self.metadata.global_node_style.background_color.clone(),
            self.metadata.style.root_node_color.clone()
        );

        self
    }

    /// Computes node style based on the mindmap global style & the node style
    /// # Arguments
    /// * `get` - function to get the node style
    /// * `set` - function to set the node style
    /// * `default_value` - default value to set if the node style is not set
    /// * `parent_value` - parent value to set if the node style is not set
    /// # Returns
    /// * `&Self` - the mindmap
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
