use crate::ast::Node;

pub struct Document{
    header: String,
    footer: String,
    content: Node // The middle rendered
}

impl Document {
    pub fn render(self) -> String {
        // Render the code and return a string
        let rendered: String =  format!("{}{}{}",self.header,self.content.render(),self.footer);
        rendered
    }
}

pub fn create_document(node: Node) -> Document {
    let doc = Document {
        header: "
<!DOCTYPE html>
<html lang='en'>
<head>
    <meta charset='UTF-8'>
    <meta name='viewport' content='width=device-width, initial-scale=1.0'>
    <title>Document</title>
</head>
        ".to_string(),

         content: node,

         footer: "
</body>
</html>
         ".to_string()};

    doc
}