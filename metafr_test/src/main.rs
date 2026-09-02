use metafr::components::br::br;
use metafr::components::typography::TypographyType::{H1, H2};
use metafr::components::{scope, typography};
use metafr::document::{Document, create_document};
use metafr::styling::style_inline::StyleTag::{ AlignItems, Display, FlexDirection, FontFamily, Margin, PaddingTop};
use metafr::styling::style_tag::{create_style_rule, style_tag_create};
use metafr::{Html, Page, get, start};

fn main() {
    let paginas = vec![Page {
        path: "/".to_string(),
        method: get(|| async {
            // Home page!
            let mut home: Document = create_document(

                //Style Tag
                &style_tag_create()
                .set_style(vec![
                    create_style_rule("*", vec![
                        Margin.get_tag("0px"),
                        FontFamily.get_tag("Arial")
                        ]),
                    create_style_rule(".center", vec![
                        Display.get_tag("flex"),
                        FlexDirection.get_tag("column"),
                        PaddingTop.get_tag("20dvh"),
                        AlignItems.get_tag("center")
                        ])
                    ],
                    
                )
                .build().render(), // Header (only the header need to render!)

                scope::container_create()
                    .add_class("center")
                    .set_children(vec![
                        typography::typography_create(H1)
                            .set_text("Hello World!")
                            .build(),
                        br(),
                        typography::typography_create(H2)
                            .set_text("🚀 You are using METAFR! 🚀")
                            .build()
                    ])
                    .build(),
            );

            Html(home.render())
        }),
    }];
    start(&paginas, vec![]);
}
