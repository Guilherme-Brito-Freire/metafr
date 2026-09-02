use crate::ast::Node;

pub struct Document {
    header: String,
    footer: String,
    content: Node, // The middle rendered
}

impl Document {
    pub fn render(&mut self) -> String {
        // Render the code and return a string
        let rendered: String = format!("{}{}{}", self.header, self.content.render(), self.footer);
        rendered
    }
}

fn get_meta_head() -> String {
    "<meta charset='UTF-8'>
    <meta name='viewport' content='width=device-width, initial-scale=1.0'>"
        .to_string()
}

pub fn create_document(head: &str,node: Node) -> Document {
    let doc = Document {
        header: format!(
            "{}{}{}{}",
            "<!DOCTYPE html>
<html lang='en'>
<head>",
            get_meta_head(),
            head,
            "</head>"
        ),

        content: node,

        footer: "
</body>
</html>
         "
        .to_string(),
    };

    doc
}
