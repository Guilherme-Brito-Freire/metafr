use std::vec;

use crate::ast::Node;

pub struct StyleTag {
    content: String,
    node: Node,
}

pub struct StyleRules{
    styles_modifiers: Vec<String>, // Like inLine module tag
    rule: String // Like: *, h1, .test::hover
}

impl StyleTag {

    // This function format the Styles structs to generate a CSS tag
    pub fn set_style(mut self, styles_rules: Vec<StyleRules>) -> StyleTag {
        let mut style: String = String::new();

        // Get the whole block
        for item in styles_rules {
            let mut modifiers: String = String::new();

            // Get all the inlines in a single String
            for modifiers_item in item.styles_modifiers {
                modifiers = format!("{}{}",modifiers,modifiers_item); // Break the line
            }
            style = format!("{}{} {} {} {}\n",style,item.rule,"{",modifiers,"}");
        }
        self.content = style; // Apply the final CSS config
        self
    }

    pub fn add_class_from_string(mut self, class_str: String) -> StyleTag {
        self.node.classes.push(class_str);
        self
    }

    pub fn build(mut self) -> Node {
        self.node.content = self.content;
        self.node
    }
}

pub fn create_style_rule(rule: &str, modifiers: Vec<String>) -> StyleRules{
    StyleRules { styles_modifiers: modifiers, rule: rule.to_string() }
}

pub fn style_tag_create() -> StyleTag {
    StyleTag {
        content: ("".to_string()),
        node: (Node {
            head_tag: format!("<{}","style"),
            content: "".to_string(),
            end_tag: format!("</{}>","style"),
            children: None,
            params: vec![],
            classes: vec![]
        }),
    }
}
