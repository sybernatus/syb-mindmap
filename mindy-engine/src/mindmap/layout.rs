use crate::mindmap::Mindmap;
use crate::mindmap::style::MindmapStyle;
use crate::node::Node;
use crate::utils::pos2::Pos2;
use crate::utils::size::Size;

pub struct LayoutEngine {}

impl LayoutEngine {

    /// Calculates the position of each nodes following the mindmap left-right horizontal layout
    pub fn layout_mindmap_left_right_horizontal(mindmap: &mut Mindmap) -> &mut Mindmap {
        let data = match mindmap.data.as_mut() {
            Some(data) => data,
            None => return mindmap,
        };

        let children = match data.children.as_mut() {
            Some(children) => children,
            None => &mut vec![],
        };

        let MindmapStyle {
            padding_horizontal,
            padding_vertical,
            ..
        } = mindmap.metadata.style;

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

        /// Recursively place the node positions based on the parent position and size
        fn place_node_positions(
            node: &mut Node,
            parent_position: Pos2,
            parent_size: Size,
            side: f32,
            horizontal_padding: f32,
            vertical_padding: f32,
        ) -> f32 {
            let mut total_height = 0.0;
            let node_size = node.graphical_size.clone().unwrap_or(Size { width: 0.0, height: 0.0 });
            let node_position_x = parent_position.x + side * (node_size.width / 2.0 + horizontal_padding + parent_size.width / 2.0);
            node.position_from_initial = Some(Pos2::new(node_position_x, parent_position.clone().y));

            if let Some(children) = &mut node.children {
                let subtree_height = node.children_graphical_size.clone().unwrap().height;
                let mut y_cursor = parent_position.y - subtree_height / 2.0;

                for child in children.iter_mut() {
                    let child_size = child.graphical_size.clone().unwrap_or(Size { width: 0.0, height: 0.0 });
                    let child_subtree = child.children_graphical_size.clone().unwrap_or(child_size.clone());

                    let child_y = y_cursor + child_subtree.height / 2.0;

                    let child_offset = Pos2 { x: node_position_x, y: child_y };
                    child.position_from_initial = Some(child_offset.clone());
                    place_node_positions(
                        child,
                        child_offset,
                        node_size.clone(),
                        side,
                        horizontal_padding,
                        vertical_padding,
                    );

                    y_cursor += child_subtree.height + vertical_padding;
                    total_height = subtree_height.max(total_height);
                }
            }
            total_height
        }

        let position_starting = Pos2::new(0.0, 0.0);

        let mut y_cursor = 0.0;
        for first_child in right_tree.iter_mut() {
            let subtree_height = first_child.children_graphical_size.clone().unwrap_or_else(|| first_child.graphical_size.clone().unwrap()).height;
            let center_y = y_cursor + subtree_height / 2.0;

            let child_pos = Pos2::new(position_starting.x, center_y);
            place_node_positions(
                first_child,
                child_pos,
                data.graphical_size.clone().unwrap(),
                1.0,
                padding_horizontal,
                padding_vertical,
            );
            y_cursor += subtree_height + padding_vertical;
        }

        let mut y_cursor = 0.0;
        for first_child in left_tree.iter_mut() {
            let subtree_height = first_child.children_graphical_size.clone().unwrap_or_else(|| first_child.graphical_size.clone().unwrap()).height;
            let center_y = y_cursor + subtree_height / 2.0;

            let child_pos = Pos2::new(position_starting.x, center_y);
            place_node_positions(
                first_child,
                child_pos,
                data.graphical_size.clone().unwrap(),
                -1.0,
                padding_horizontal,
                padding_vertical,
            );
            y_cursor += subtree_height + padding_vertical;
        }

        let mut min_y = f32::MAX;
        let mut max_y = f32::MIN;

        for child in children.iter() {
            if let Some(pos) = &child.position_from_initial {
                let size = child.graphical_size.clone().unwrap_or(Size { width: 0.0, height: 0.0 });
                min_y = min_y.min(pos.y - size.height / 2.0);
                max_y = max_y.max(pos.y + size.height / 2.0);
            }
        }

        let center_y = (min_y + max_y) / 2.0;
        mindmap.data.as_mut().unwrap().position_from_initial = Some(Pos2 { x: 0.0, y: center_y });

        mindmap

    }

    /// Calculates the position of each nodes following the mindmap standard layout
    pub fn layout_mindmap_standard(mindmap: &mut Mindmap) -> &mut Mindmap  {
        let graphical_size = match mindmap.data.clone() {
            Some(data) => data.get_graphical_size(),
            None => Size::default(),
        };

        let data = match mindmap.data.as_mut() {
            Some(data) => data,
            None => return mindmap,
        };

        let children = match data.children.as_mut() {
            Some(children) => children,
            None => &mut vec![],
        };


        let MindmapStyle {
            padding_horizontal,
            padding_vertical,
            ..
        } = mindmap.metadata.style;

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
        mindmap.data.as_mut().unwrap().position_from_initial = Some(Pos2 {
            x: position_starting.x,
            y: position_starting.y + total_height / 2.0 - graphical_size.height / 2.0,
        });

        mindmap
    }
}