use crate::ast::Node;
use crate::param::Param;

pub struct Struture {
    children: Vec<Node>,
    node: Node,
}

pub enum StrutureType{
    Aside,
    Footer,
    Section,
    Nav,
    Header
}

impl Struture {

    pub fn set_children(mut self, children: Vec<Node>) -> Struture {
        self.children = children;
        self
    }

    pub fn set_params(mut self, params: Vec<Param>) -> Struture {
        self.node.params = params;
        self
    }

    pub fn add_class(mut self, class_str: &str) -> Struture {
        self.node.classes.push(class_str.to_string());
        self
    }

    pub fn add_class_from_string(mut self, class_str: String) -> Struture {
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

impl StrutureType {
    fn get(&self) -> String {
        match self {
            StrutureType::Aside => "aside".to_string(),
            StrutureType::Footer => "footer".to_string(),
            StrutureType::Header => "header".to_string(),
            StrutureType::Nav => "nav".to_string(),
            StrutureType::Section => "section".to_string(),
        }
    }
}

// Create
pub fn structure_create(structure_type: StrutureType) -> Struture { // Create the base model
    Struture {
        children: Vec::new(),
        node: (Node {
            head_tag: format!("{}{}","<",structure_type.get()),
            content: "".to_string(),
            end_tag: format!("{}{}{}","</",structure_type.get(),">"),
            params: vec![],
            classes: vec![],
        }),
    }
}