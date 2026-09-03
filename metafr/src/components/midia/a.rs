use crate::ast::Node;
use crate::param::{Param, create_param};

pub struct A {
    children: Vec<Node>,
    href: String,
    node: Node,
}

impl A {

    pub fn set_children(mut self, children: Vec<Node>) -> A {
        self.children = children;
        self
    }


    pub fn set_href(mut self, href: &str) -> A {
        self.href = href.to_string();
        self
    }

    pub fn set_params(mut self, params: Vec<Param>) -> A {
        self.node.params = params;
        self
    }

    pub fn add_class(mut self, class_str: &str) -> A {
        self.node.classes.push(class_str.to_string());
        self
    }

    pub fn add_class_from_string(mut self, class_str: String) -> A {
        self.node.classes.push(class_str);
        self
    }

    pub fn build(mut self) -> Node {
        self.node.params.push(create_param("href", &self.href)); // Image Source

        // Content
        let mut content: String = String::new();
        for item in &mut self.children {
            content.push_str(&item.render());
        }
        self.node.content = content;

        self.node
    }
}

pub fn a_create() -> A {
    A {
        href: "no content".to_string(),
        node: (Node {
            head_tag: "<a".to_string(),
            content: "".to_string(),
            end_tag: "</a>".to_string(),
            params: vec![],
            classes: vec![]
        }),
        children: vec![],
    }
}
