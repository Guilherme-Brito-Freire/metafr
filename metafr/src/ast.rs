use crate::param::{Param, get_param_list};

pub struct Node {
    pub(crate) head_tag: String,
    pub(crate) end_tag: String,
    pub(crate) content: String,
    pub(crate) params: Vec<Param>,
    pub(crate) children: Option<Vec<Node>>
}

impl Node{
    pub fn render(&self) -> String{
        format!("{}{}{}",get_head_tag(self),self.content,self.end_tag)
    }
}

// This function apply the params inside the tag
fn get_head_tag(node: &Node) -> String {
    return format!("{}{}>",node.head_tag,get_param_list(&node.params))
}