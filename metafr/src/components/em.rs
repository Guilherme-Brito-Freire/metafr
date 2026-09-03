use crate::ast::Node;

// This component Break the Line
pub fn em() -> Node {
    Node { head_tag: "<em".to_string(), end_tag: String::new(), content: String::new(), params: vec![], classes: vec![] }
}