use crate::layout::Layout;
use crate::mindmap::Mindmap;
use crate::mindmap::style::MindmapStyle;
use crate::node::Node;
use crate::layout::pos2::Pos2;
use crate::layout::size::Size;

pub struct LeftRightBottomLayout {}


impl Layout for LeftRightBottomLayout {}
impl LeftRightBottomLayout {

    /// Recursively place the node positions based on the parent position and size
    /// # Arguments
    /// * `current_tree` - The actual tree to position
    /// * `parent_position` - The position of the parent node
    /// * `parent_size` - The size of the parent node
    /// * `side` - The side of the tree (-1 for left, 1 for right)
    /// * `horizontal_padding` - The horizontal padding between nodes
    /// * `vertical_padding` - The vertical padding between nodes
    /// # Returns
    /// * `f32` - The total height of the subtree
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
                    Self::layout_mindmap_standard_children(
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

    /// Calculates the position of the parent node based on its children
    /// # Arguments
    /// * `position_starting` - The initial position of the parent
    /// * `total_height` - The total height of the mindmap
    /// * `graphical_size` - The graphical size of the parent node
    /// # Returns
    /// * `Pos2` - The position of the parent node
    fn position_parent_node(
        position_starting: Pos2,
        total_height: f32,
        graphical_size: Size
    ) -> Pos2 {

        // Center parent node on children
        Pos2 {
            x: position_starting.x,
            y: position_starting.y + total_height / 2.0 - graphical_size.height / 2.0,
        }
    }

    /// Calculates the position of each nodes following the mindmap standard layout
    pub fn layout(mindmap: &mut Mindmap) -> &mut Mindmap  {
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
        let (right_tree,left_tree) = Self::divide_elements_tree(children);

        let position_starting = Pos2::new(0.0, 0.0);

        // Layout right tree
        let right_height = Self::layout_mindmap_standard_children(
            right_tree,
            position_starting.clone(),
            graphical_size.clone(),
            1.0,
            padding_horizontal,
            padding_vertical,
        );

        // Layout left tree
        let left_height = Self::layout_mindmap_standard_children(
            left_tree,
            position_starting.clone(),
            graphical_size.clone(),
            -1.0,
            padding_horizontal,
            padding_vertical,
        );

        let total_height = right_height.max(left_height);
        let parent_position = Self::position_parent_node(position_starting, total_height, graphical_size);
        mindmap.data.as_mut().unwrap().position_from_initial = Some(parent_position);
        mindmap
    }
}