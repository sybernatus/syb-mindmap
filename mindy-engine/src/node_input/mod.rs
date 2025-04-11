use serde::Deserialize;
use std::cell::RefCell;

#[derive(Debug, Clone, Default, PartialEq, Deserialize)]
pub struct Pos2 {
    pub x: f32,
    pub y: f32,
}

impl Pos2 {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct Rgb {
    pub red: f32,
    pub green: f32,
    pub blue: f32,
}

impl Rgb {
    pub fn new(red: f32, green: f32, blue: f32) -> Self {
        Self { red, green, blue }
    }
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct NodeStyleCustom {
    pub background_color: Rgb,
    pub children_hidden: bool,
    pub font_size: f32,
    pub font_family: String,
    pub max_width: f32,
    pub min_width: f32,
    pub min_height: f32,
    pub padding: f32,
    pub text_wrapping: bool,
}

impl Default for NodeStyleCustom {
    fn default() -> Self {
        Self {
            background_color: Rgb::new(122.0, 10.0, 0.0),
            children_hidden: false,
            text_wrapping: true,
            font_size: 12.0,
            padding: 10.0,
            font_family: "'Segoe UI', Tahoma, Geneva, Verdana, sans-serif".to_string(),
            max_width: 200.0,
            min_width: 0.0,
            min_height: 0.0,
        }
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct Size {
    pub width: f32,
    pub height: f32,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Default)]
pub enum Direction  {
    #[default]
    Right,
    Left,
}


#[derive(Debug, Clone, Default, PartialEq, Deserialize)]
pub struct NodeInput {
    pub text: Option<String>,
    pub style_custom: Option<NodeStyleCustom>,
    pub position_direction: Option<Direction>,
    pub children: Option<Vec<NodeInput>>,
    pub position: Option<Pos2>,
}

impl NodeInput {
    pub fn new() -> Self {
        Self {
            text: None,
            children: None,
            position_direction: Option::from(Direction::default()),
            style_custom: Option::from(NodeStyleCustom::default()),
            position: None,
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

    pub fn with_position(&mut self, position: Pos2) -> Self {
        self.position = Some(position);
        self.clone()
    }

    pub fn get_children(&self) -> Option<Vec<NodeInput>> {
        self.children.clone()
    }


    pub fn get_graphical_text_size(&self) -> Size {

        let NodeStyleCustom {
            max_width,
            text_wrapping,
            font_size,
            ..
        } = self.style_custom.clone().unwrap_or_else(|| NodeStyleCustom::default());

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


        let NodeStyleCustom {
            min_width,
            max_width,
            min_height,
            text_wrapping,
            padding, ..
        } = self.style_custom.clone().unwrap_or_else(|| NodeStyleCustom::default());

        let Size {
            height: text_height,
            width: text_width
        } = self.get_graphical_text_size();

        let new_width = match text_width + (padding * 2.0) {
            size if size < min_width => min_width,
            size if size > max_width && text_wrapping  => max_width,
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

    pub fn layout_mindmap_center(&mut self) {
        const H_PADDING: f32 = 50.0; // Horizontal padding between levels
        const V_PADDING: f32 = 20.0;

        self.position = Some(Pos2::new(200.0, 200.0));

        // Divide children into left and right based on their position direction
        let mut left = vec![];
        let mut right = vec![];

        let Size {
            width,
            height
        } = self.get_graphical_size();

        for mut child in self.children.as_mut().unwrap() {
            match child.position_direction {
                Some(Direction::Left) => left.push(child),
                _ => right.push(child),
            }
        }

        fn layout_children(mut children: Vec<&mut NodeInput>, x_offset: f32, parent_node_position: Pos2, parent_node_size: Size) -> Vec<&mut NodeInput> {
            let mut y_cursor = 0.0;
            for child in children.iter_mut() {
                let subtree_height = NodeInput::layout_subtree(child, x_offset, y_cursor, parent_node_position.clone(), parent_node_size.clone());
                y_cursor += subtree_height + V_PADDING;
            }
            children
        };

        let left_offset = -width - H_PADDING;
        let right_offset = width + H_PADDING;
        let _left_children = layout_children(left, left_offset, self.position.clone().unwrap(), Size::from(Size { width, height }));
        let _right_children = layout_children(right, right_offset, self.position.clone().unwrap(), Size::from(Size { width, height }));

        self.layout_parent_position(self.position.clone().unwrap().x);
    }

    fn layout_subtree(
        &mut self,
        offset_x: f32,
        offset_y: f32,
        initial_position: Pos2,
        parent_size: Size,
    ) -> f32 {
        const H_PADDING: f32 = 50.0;
        const V_PADDING: f32 = 20.0;

        let Size {
            width,
            height
        } = self.get_graphical_size();

        let node_half_width = width / 2.0;
        let parent_half_width = parent_size.width / 2.0;
        let node_offset_x = offset_x + node_half_width + parent_half_width + initial_position.x;

        let node_half_height = height / 2.0;
        let parent_half_height = parent_size.height / 2.0;
        let node_offset_y = offset_y + node_half_height + parent_half_height + initial_position.y;

        if self.children.as_mut().unwrap().is_empty() {
            self.position = Some(Pos2::new(node_offset_x, node_offset_y));
            return height;
        }

        let mut y_cursor = offset_y;
        let mut total_height = 0.0;

        for child in self.children.as_mut().unwrap() {
            let new_x = offset_x + match self.position_direction.as_mut() {
                Some(Direction::Left) => -width - H_PADDING + initial_position.x,
                _ => width + H_PADDING + initial_position.x,
            };
            let subtree_height = Self::layout_subtree(
                child,
                new_x,
                y_cursor,
                initial_position.clone(),
                Size::from(Size { width, height }),
            );
            y_cursor += subtree_height + V_PADDING;
            total_height += subtree_height + V_PADDING;
        }

        self.layout_parent_position(node_offset_x);
        tracing::debug!("text: {:?} - self.position.x: {:?} - self.position.y: {:?}", self.text, self.position.clone().unwrap().x, self.position.clone().unwrap().y);

        total_height.max(height)
    }

    fn layout_parent_position(
        &mut self,
        parent_position_x: f32,
    ) {
        let children = self.children.as_mut().unwrap();
        let fist_child = children.first().unwrap();
        let last_child = children.last().unwrap();

        let children_middle = children.len() / 2;

        let first_y = fist_child.clone().position.unwrap().y;
        let last_y = last_child.clone().position.unwrap().y;
        tracing::debug!("text: {:?} - first_y: {:?} - last_y: {:?}", self.text, first_y, last_y);

        self.position = Some(Pos2::new(parent_position_x, children[children_middle].clone().position.unwrap().y));
    }
}