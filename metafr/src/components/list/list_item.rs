use crate::ast::Node;
use crate::param::Param;

pub struct ListItem {
    children: Vec<Node>,
    node: Node,
}

pub enum ListItemType{
    Li,
    Dl
}

impl ListItem {
    pub fn set_children(mut self, children: Vec<Node>) -> ListItem {
        self.children = children;
        self
    }

    pub fn set_params(mut self, params: Vec<Param>) -> ListItem {
        self.node.params = params;
        self
    }

    pub fn add_class(mut self, class_str: &str) -> ListItem {
        self.node.classes.push(class_str.to_string());
        self
    }

    pub fn add_class_from_string(mut self, class_str: String) -> ListItem {
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

impl ListItemType{
    pub fn get(&self) -> String {
        match &self {
            ListItemType::Li => "li".to_string(),
            ListItemType::Dl => "dl".to_string(),
        }
    }
}

// Create the Base and Variations
pub fn list_item_create(list_item_type: ListItemType) -> ListItem {
    // Create the base model
    ListItem {
        children: Vec::new(),
        node: (Node {
            head_tag: format!("{}{}","<",list_item_type.get()),
            content: "".to_string(),
            end_tag: format!("{}{}{}","</",list_item_type.get(),">"),
            params: vec![],
            classes: vec![],
        }),
    }
}