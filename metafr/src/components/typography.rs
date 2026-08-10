use crate::ast::Node;

pub struct Typography {
    text: String,
    node: Node,
}

pub fn typography_create() -> Typography {
    Typography {
        text: (
            "Hello World!".to_string()
        ),
        node: (Node {
            head_tag: "<h1>".to_string(),
            content: "".to_string(),
            end_tag: "</h1>".to_string(),
        }),
    }
}

impl Typography {

    pub fn set_text(mut self, text: &str) -> Typography {
        self.text=text.to_string();
        self
    }

    pub fn build(mut self) -> Node {
        self.node.content = self.text;
        self.node
    }
}