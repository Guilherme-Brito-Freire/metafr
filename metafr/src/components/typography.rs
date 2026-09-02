use crate::ast::Node;
use crate::param::Param;

pub struct Typography {
    text: String,
    node: Node,
}

pub enum TypographyType {
    H1,
    H2,
    H3,
    H4,
    H5,
    H6,
    P,
    Span,
}

impl Typography {
    pub fn set_text(mut self, text: &str) -> Typography {
        self.text = text.to_string();
        self
    }

    pub fn set_params(mut self, params: Vec<Param>) -> Typography {
        self.node.params = params;
        self
    }

    pub fn add_class(mut self, class_str: &str) -> Typography {
        self.node.classes.push(class_str.to_string());
        self
    }

    pub fn add_class_from_string(mut self, class_str: String) -> Typography {
        self.node.classes.push(class_str);
        self
    }

    pub fn build(mut self) -> Node {
        self.node.content = self.text;
        self.node
    }
}

// Show the avaible options to typography
impl TypographyType {
    fn get_tag_open(&self) -> String {
        match self {
            TypographyType::H1 => "h1".to_string(),
            TypographyType::H2 => "h2".to_string(),
            TypographyType::H3 => "h3".to_string(),
            TypographyType::H4 => "h4".to_string(),
            TypographyType::H5 => "h5".to_string(),
            TypographyType::H6 => "h6".to_string(),
            TypographyType::P => "p".to_string(),
            TypographyType::Span => "span".to_string(),
        }
    }
}

pub fn typography_create(typography_type: TypographyType) -> Typography {
    Typography {
        text: ("Hello World!".to_string()),
        node: (Node {
            head_tag: format!("<{}",typography_type.get_tag_open()),
            content: "".to_string(),
            end_tag: format!("</{}>",typography_type.get_tag_open()),
            children: None,
            params: vec![],
            classes: vec![]
        }),
    }
}
