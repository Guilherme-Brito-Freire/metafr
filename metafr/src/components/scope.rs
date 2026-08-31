use crate::ast::Node;
use crate::param::Param;

pub struct Scope {
    children: Vec<Node>,
    node: Node,
}

pub fn scope_create() -> Scope {
    Scope {
        children: Vec::new(),
        node: (Node {
            head_tag: "<div".to_string(),
            content: "".to_string(),
            end_tag: "</div>".to_string(),
            children: None,
            params: vec![],
        }),
    }
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

    pub fn build(mut self) -> Node {
        let mut content: String = String::new();
        for item in &self.children{
            content.push_str(&item.render());
        }
        self.node.content = content;
        self.node.children = Some(self.children);
        self.node
    }
}