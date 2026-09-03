use crate::ast::Node;
use crate::param::{Param, create_param};

pub struct Img {
    src: String,
    alt: String,
    node: Node,
}

impl Img {
    pub fn set_src(mut self, src: &str) -> Img {
        self.src = src.to_string();
        self
    }

    pub fn set_alt(mut self, alt: &str) -> Img {
        self.alt = alt.to_string();
        self
    }

    pub fn set_params(mut self, params: Vec<Param>) -> Img {
        self.node.params = params;
        self
    }

    pub fn add_class(mut self, class_str: &str) -> Img {
        self.node.classes.push(class_str.to_string());
        self
    }

    pub fn add_class_from_string(mut self, class_str: String) -> Img {
        self.node.classes.push(class_str);
        self
    }

    pub fn build(mut self) -> Node {
        self.node.params.push(create_param("src", &self.src)); // Image Source
        self.node.params.push(create_param("alt", &self.alt)); // Image alt
        self.node
    }
}

pub fn img_create() -> Img {
    Img {
        src: "no content".to_string(),
        alt: "image".to_string(),
        node: (Node {
            head_tag: "<img".to_string(),
            content: "".to_string(),
            end_tag: "</img>".to_string(),
            params: vec![],
            classes: vec![]
        }),
    }
}
