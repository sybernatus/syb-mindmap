use palette::num::Round;
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
            text_wrapping: false,
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

#[derive(Debug, Clone, Default, PartialEq)]
pub struct Node {
    pub id: i32,
    pub content: NodeContent,
    pub position: Pos2,
    pub style_custom: NodeStyleCustom,
    pub parent_id: Option<i32>,
}

impl Node {

    pub fn new() -> Self {
        Self {
            id: 0,
            content: NodeContent::default(),
            position: Pos2::new(0.0, 0.0),
            style_custom: NodeStyleCustom::default(),
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
        self.style_custom.background_color = Rgb::new(r, g, b);
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

    pub fn with_font_size(&mut self, font_size: f32) -> Self {
        self.style_custom.font_size = font_size;
        self.clone()
    }

    pub fn with_text_wrapping(&mut self, text_wrapping: bool) -> Self {
        self.style_custom.text_wrapping = text_wrapping;
        self.clone()
    }

    pub fn with_style_custom(&mut self, style_custom: NodeStyleCustom) -> Self {
        self.style_custom = style_custom;
        self.clone()
    }

    pub fn get_graphical_text_size(&self) -> Size {

        let NodeStyleCustom {
            max_width,
            text_wrapping,
            font_size,
            ..
        } = self.style_custom;

        let font_char_width = font_size - 4.0;
        let font_char_height = font_size - 2.0;
        let font_interline = 4.0;

        let text_length = self.content.text.as_ref().map_or(0, |text| text.len()) as f32;
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
        } = self.style_custom;

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
}