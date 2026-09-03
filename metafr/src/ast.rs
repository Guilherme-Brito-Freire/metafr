use crate::param::{Param, create_param, get_param_list};

pub struct Node {
    pub(crate) head_tag: String,
    pub(crate) end_tag: String,
    pub(crate) content: String,
    pub(crate) params: Vec<Param>,
    pub(crate) classes: Vec<String>
}

impl Node{
    pub fn render(&mut self) -> String{
        if self.classes.len() != 0 { // This if only write classe if exist at least one
            let mut classes_str: String = String::new(); //inicialize
            for item in &self.classes {
                classes_str = format!("{} {}",classes_str,item);
            }
            self.params.push(create_param("class", &classes_str));
        }
        
        // Format the final result
        format!("{}{}{}",get_head_tag(self),self.content,self.end_tag)
    }
}

// This function apply the params inside the tag
fn get_head_tag(node: &Node) -> String {
    return format!("{}{}>",node.head_tag,get_param_list(&node.params))
}