use crate::ast::Node;

// This component Break the Line
pub fn br() -> Node {
    Node { head_tag: "<br".to_string(), end_tag: String::new(), content: String::new(), params: vec![], classes: vec![] }
}