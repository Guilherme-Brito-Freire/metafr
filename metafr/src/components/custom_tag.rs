use crate::ast::Node;
use crate::param::Param;

pub struct CustomTag {
    content: String,
    node: Node,
}

impl CustomTag {
    pub fn set_content(mut self, text: &str) -> CustomTag {
        self.content = text.to_string();
        self
    }

    pub fn set_params(mut self, params: Vec<Param>) -> CustomTag {
        self.node.params = params;
        self
    }

    pub fn build(mut self) -> Node {
        self.node.content = self.content;
        self.node
    }
}

pub fn custom_tag_create(tag: &str) -> CustomTag {
    CustomTag {
        content: ("({[Here you need to put some content]})".to_string()),
        node: (Node {
            head_tag: format!("<{}",tag.to_string()),
            content: "".to_string(),
            end_tag: format!("</{}>",tag.to_string()),
            children: None,
            params: vec![],
        }),
    }
}
