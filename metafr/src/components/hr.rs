use crate::ast::Node;

// This component Break the Line
pub fn hr() -> Node {
    Node { head_tag: "<hr".to_string(), end_tag: String::new(), content: String::new(), params: vec![], classes: vec![] }
}