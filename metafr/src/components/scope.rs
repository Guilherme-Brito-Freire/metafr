use crate::ast::Node;
use crate::param::Param;

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
        self.node.children = Some(self.children);
        self.node
    }
}

// Create the Base and Variations

pub fn scope_create() -> Scope { // Create the base model
    Scope {
        children: Vec::new(),
        node: (Node {
            head_tag: "<div".to_string(),
            content: "".to_string(),
            end_tag: "</div>".to_string(),
            children: None,
            params: vec![],
            classes: vec![],
        }),
    }
}

pub fn container_create() -> Scope { // Container, with margin etc
    Scope {
        children: Vec::new(),
        node: (Node {
            head_tag: "<div".to_string(),
            content: "".to_string(),
            end_tag: "</div>".to_string(),
            children: None,
            params: vec![],
            classes: vec!["container".to_string()],
        }),
    }
}