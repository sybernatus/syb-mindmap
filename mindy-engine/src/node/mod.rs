use palette::rgb::Rgb;

#[derive(Debug, Clone, Default, PartialEq)]
pub struct Pos2 (pub f32, pub f32);

impl Pos2 {
    pub fn new(x: f32, y: f32) -> Self {
        Self(x, y)
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
    pub hidden: bool,
}

impl Default for NodeStyleCustom {
    fn default() -> Self {
        Self {
            color: Rgb::new(0.0, 0.0, 0.0),
            hidden: false,
        }
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct Node {
    pub content: NodeContent,
    pub position: Pos2,
    pub style_custom: NodeStyleCustom,
}

impl Node {

    pub fn new() -> Self {
        Self {
            content: NodeContent::default(),
            position: Pos2::new(0.0, 0.0),
            style_custom: NodeStyleCustom::default(),
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
        self.style_custom.hidden = hidden;
        self.clone()
    }

    pub fn with_color(&mut self, r: f32, g: f32, b: f32) -> Self {
        self.style_custom.color = Rgb::new(r, g, b);
        self.clone()
    }
}