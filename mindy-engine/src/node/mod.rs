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
}

impl Default for NodeStyleCustom {
    fn default() -> Self {
        Self {
            color: Rgb::new(122.0, 10.0, 0.0),
            children_hidden: false,
        }
    }
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
}