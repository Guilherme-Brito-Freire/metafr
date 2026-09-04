use crate::ast::Node;
use crate::param::Param;
use crate::styling::style_inline::StyleTag::{AlignItems, BackgroundColor, BoxSizing, Color, Display, FlexDirection, Height, PaddingLeft, PaddingRight, Width};
use crate::styling::style_inline::get_style_inline;

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

// Variations
pub fn nav_styled_create(color: &str) -> Struture { // Create the base model
    Struture {
        children: Vec::new(),
        node: (Node {
            head_tag: "<nav".to_string(),
            content: "".to_string(),
            end_tag: "</nav>".to_string(),
            params: vec![],
            classes: vec![],
        }),
    }.set_params(vec![get_style_inline(vec![
        Width.get_tag("100%"),
        Height.get_tag("65px"),
        BackgroundColor.get_tag(color),
        Display.get_tag("flex"),
        FlexDirection.get_tag("row"),
        AlignItems.get_tag("center"),
        PaddingLeft.get_tag("20px"),
        PaddingRight.get_tag("20px"),
        BoxSizing.get_tag("border-box"),
        Color.get_tag("white"),
    ])])
}

// Variations
pub fn footer_styled_create(color: &str) -> Struture { // Create the base model
    Struture {
        children: Vec::new(),
        node: (Node {
            head_tag: "<footer".to_string(),
            content: "".to_string(),
            end_tag: "</footer>".to_string(),
            params: vec![],
            classes: vec![],
        }),
    }.set_params(vec![get_style_inline(vec![
        Width.get_tag("100%"),
        Height.get_tag("65px"),
        BackgroundColor.get_tag(color),
        Display.get_tag("flex"),
        FlexDirection.get_tag("row"),
        AlignItems.get_tag("center"),
        PaddingLeft.get_tag("20px"),
        PaddingRight.get_tag("20px"),
        BoxSizing.get_tag("border-box"),
        Color.get_tag("white"),
    ])])
}