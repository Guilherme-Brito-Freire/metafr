use crate::ast::Node;
use crate::param::Param;
use crate::styling::style_inline;
use crate::styling::style_inline::StyleTag::{BoxSizing, MaxWidth, Padding, PaddingLeft, PaddingRight, Width};

pub struct Scope {
    children: Vec<Node>,
    node: Node,
}

impl Scope {
    pub fn set_children(mut self, children: Vec<Node>) -> Scope {
        self.children = children;
        self
    }

    pub fn set_params(mut self, params: Vec<Param>) -> Scope {
        self.node.params = params;
        self
    }

    pub fn add_class(mut self, class_str: &str) -> Scope {
        self.node.classes.push(class_str.to_string());
        self
    }

    pub fn add_class_from_string(mut self, class_str: String) -> Scope {
        self.node.classes.push(class_str);
        self
    }

    pub fn build(mut self) -> Node {
        let mut content: String = String::new();
        for item in &mut self.children {
            content.push_str(&item.render());
        }
        self.node.content = content;
        self.node
    }
}

// Create the Base and Variations

pub fn scope_create() -> Scope {
    // Create the base model
    Scope {
        children: Vec::new(),
        node: (Node {
            head_tag: "<div".to_string(),
            content: "".to_string(),
            end_tag: "</div>".to_string(),
            params: vec![],
            classes: vec![],
        }),
    }
}

// Implement a default style
pub fn container_create() -> Scope {
    // Container, with margin etc
    Scope {
        children: Vec::new(),
        node: (Node {
            head_tag: "<div".to_string(),
            content: "".to_string(),
            end_tag: "</div>".to_string(),
            params: vec![],
            classes: vec!["container".to_string()],
        }),
    }
    .set_params(vec![style_inline::get_style_inline(vec![
        Padding.get_tag("5px"),
        PaddingLeft.get_tag("20%"),
        PaddingRight.get_tag("20%"),
        Width.get_tag("100%"),
        MaxWidth.get_tag("100%"),
        BoxSizing.get_tag("border-box")
    ])])
}
