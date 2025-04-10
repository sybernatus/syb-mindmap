use palette::rgb::Rgb;

#[derive(Debug, Clone, Default, PartialEq)]
pub struct Pos2 {
    pub x: f32,
    pub y: f32,
}

impl Pos2 {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct NodeContent {
    pub text: Option<String>,
    pub image: Option<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct NodeStyleCustom {
    pub color: Rgb,
    pub children_hidden: bool,
    pub text_wrapping: bool,
    pub font_size: f32,
    pub padding: f32,
}

impl Default for NodeStyleCustom {
    fn default() -> Self {
        Self {
            color: Rgb::new(122.0, 10.0, 0.0),
            children_hidden: false,
            text_wrapping: false,
            font_size: 12.0,
            padding: 10.0,
        }
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct NodeSize {
    pub width: f32,
    pub height: f32,
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct Node {
    pub id: i32,
    pub content: NodeContent,
    pub position: Pos2,
    pub style_custom: NodeStyleCustom,
    pub size: Option<NodeSize>,
    pub parent_id: Option<i32>,
}

impl Node {

    pub fn new() -> Self {
        Self {
            id: 0,
            content: NodeContent::default(),
            position: Pos2::new(0.0, 0.0),
            style_custom: NodeStyleCustom::default(),
            size: None,
            parent_id: None,
        }
    }

    pub fn with_text(&mut self, text: String) -> Self {
        self.content.text = Some(text);
        self.clone()
    }

    pub fn with_content(&mut self, content: NodeContent) -> Self {
        self.content = content;
        self.clone()
    }

    pub fn with_position(&mut self, position: Pos2) -> Self {
        self.position = position;
        self.clone()
    }

    pub fn hidden(&mut self, hidden: bool) -> Self {
        self.style_custom.children_hidden = hidden;
        self.clone()
    }

    pub fn with_color(&mut self, r: f32, g: f32, b: f32) -> Self {
        self.style_custom.color = Rgb::new(r, g, b);
        self.clone()
    }

    pub fn with_id(&mut self, id: i32) -> Self {
        self.id = id;
        self.clone()
    }

    pub fn set_parent(&mut self, parent_id: i32) -> Self {
        self.parent_id = Some(parent_id);
        self.clone()
    }

    pub fn get_size(&self) -> Option<NodeSize> {
        self.size.clone()
    }

    pub fn with_font_size(&mut self, font_size: f32) -> Self {
        self.style_custom.font_size = font_size;
        self.clone()
    }

    pub fn with_text_wrapping(&mut self, text_wrapping: bool) -> Self {
        self.style_custom.text_wrapping = text_wrapping;
        self.clone()
    }

    pub fn with_graphical_size(&mut self) -> Self {

        // Calculate the size of the node based on its content
        let _font_type = "Arial";
        let font_size = self.style_custom.font_size;
        let text_wrapping = self.style_custom.text_wrapping;
        let padding = self.style_custom.padding;
        let min_width = 200.0;
        let min_height = 0.0;
        let max_width = 300.0;

        let text_length = self.content.text.as_ref().map_or(0, |text| text.len()) as f32;
        let text_size = text_length * font_size;
        let new_width = match text_size {
            size if size < min_width => min_width,
            size if size > max_width && text_wrapping  => max_width,
            size => size,
        };

        let new_height = if text_wrapping {
            // Calculate the number of lines needed for the text
            let lines = (text_size / max_width).ceil() + 1.0;
            let new_height = if lines * font_size > min_height {
                lines * font_size
            } else {
                min_height
            };
            new_height
        } else {
            if font_size < min_height {
                min_height
            } else {
                font_size
            }
        };

        tracing::debug!("Node size: {:?} - {:?}", new_width, new_height);
        let new_size = NodeSize {
            width: new_width + padding * 2.0,
            height: new_height + padding * 2.0,
        };

        self.size = Some(new_size);
        self.clone()
    }
}