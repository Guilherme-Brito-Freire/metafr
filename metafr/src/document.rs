
/*
pub struct Node{
    head_tag: String,
    end_tag: String,
    children: Vec<Node>,
    content: String
}
*/

pub struct Document{
    header: String,
    footer: String,
    content: String // The middle rendered
}

impl Document {
    pub fn render(&self) -> String {
        // Render the code and return a string
        let rendered: String =  format!("{}{}{}",self.header,self.content,self.footer);
        rendered
    }
}

pub fn create_document() -> Document {
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
         content: "
         
         <h1>Hello World!</h1>

         ".to_string(),
         footer: "
</body>
</html>
         ".to_string()};

    doc
}