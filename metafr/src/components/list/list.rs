use crate::ast::Node;
use crate::param::Param;

pub struct List {
    children: Vec<Node>,
    node: Node,
}

pub enum ListType{
    Ul,
    Ol,
    Dl
}

impl List {
    pub fn set_children(mut self, children: Vec<Node>) -> List {
        self.children = children;
        self
    }

    pub fn set_params(mut self, params: Vec<Param>) -> List {
        self.node.params = params;
        self
    }

    pub fn add_class(mut self, class_str: &str) -> List {
        self.node.classes.push(class_str.to_string());
        self
    }

    pub fn add_class_from_string(mut self, class_str: String) -> List {
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

impl ListType{
    pub fn get(&self) -> String {
        match &self {
            ListType::Ul => "ul".to_string(),
            ListType::Ol => "ol".to_string(),
            ListType::Dl => "dl".to_string(),
        }
    }
}

// Create the Base and Variations
pub fn list_create(list_type: ListType) -> List {
    // Create the base model
    List {
        children: Vec::new(),
        node: (Node {
            head_tag: format!("{}{}","<",list_type.get()),
            content: "".to_string(),
            end_tag: format!("{}{}{}","</",list_type.get(),">"),
            params: vec![],
            classes: vec![],
        }),
    }
}