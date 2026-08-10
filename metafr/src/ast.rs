pub struct Node {
    pub(crate) head_tag: String,
    pub(crate) end_tag: String,
    pub(crate) content: String,
}

impl Node{
    pub fn render(self) -> String{
        format!("{}{}{}",self.head_tag,self.content,self.end_tag)
    }
}