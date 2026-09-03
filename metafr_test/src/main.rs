use metafr::components::br::br;
use metafr::components::custom_tag::custom_tag_create;
use metafr::components::em::em;
use metafr::components::hr::hr;
use metafr::components::media::{a::a_create, img::img_create};
use metafr::components::scope::{container_create, scope_create};
use metafr::components::structure::{StrutureType, structure_create};
use metafr::components::typography::{
    TypographyType::{H1, H2, H3, P, Span},
    typography_create,
};
use metafr::document::{Document, create_document};
use metafr::styling::style_inline::StyleTag::{
    AlignItems, Display, FlexDirection, FontFamily, Gap, Margin, Padding, PaddingTop,
};
use metafr::styling::style_tag::{create_style_rule, style_tag_create};
use metafr::{Html, Page, get, start, static_serve};

fn main() {
    let paginas = vec![Page {
        path: "/".to_string(),
        method: get(|| async {
            // Home page!
            let mut home: Document = create_document(
                // Style Tag
                &style_tag_create()
                    .set_style(vec![
                        create_style_rule(
                            "*",
                            vec![Margin.get_tag("0px"), FontFamily.get_tag("Arial")],
                        ),
                        create_style_rule(
                            ".center",
                            vec![
                                Display.get_tag("flex"),
                                FlexDirection.get_tag("column"),
                                PaddingTop.get_tag("5dvh"),
                                AlignItems.get_tag("center"),
                                Gap.get_tag("20px"),
                            ],
                        ),
                        create_style_rule(".box", vec![Padding.get_tag("20px")]),
                    ])
                    .build()
                    .render(),
                container_create()
                    .add_class("center")
                    .set_children(vec![
                        structure_create(StrutureType::Header)
                            .set_children(vec![
                                typography_create(H1)
                                    .set_text("MetaFR Framework Components Test")
                                    .build(),
                            ])
                            .build(),
                        hr(),
                        structure_create(StrutureType::Section)
                            .add_class("box")
                            .set_children(vec![
                                typography_create(H2).set_text("Typography").build(),
                                typography_create(H3).set_text("This is an H3 tag").build(),
                                typography_create(P)
                                    .set_text("This is a paragraph.")
                                    .build(),
                                typography_create(Span).set_text("This is a span.").build(),
                            ])
                            .build(),
                        hr(),
                        structure_create(StrutureType::Section)
                            .add_class("box")
                            .set_children(vec![
                                typography_create(H2)
                                    .set_text("Simple Tags (hr, br, em)")
                                    .build(),
                                typography_create(P).set_text("Text before br").build(),
                                br(),
                                typography_create(P)
                                    .set_text("Text after br, and here is an em tag:")
                                    .build(),
                                em(), // Actually em is broken and doesn't wrap content, just outputs <em
                            ])
                            .build(),
                        hr(),
                        structure_create(StrutureType::Section)
                            .add_class("box")
                            .set_children(vec![
                                typography_create(H2).set_text("Media").build(),
                                img_create()
                                    .set_src("/assets/images.jpeg")
                                    .set_alt("Test Image from assets")
                                    .build(),
                                a_create()
                                    .set_href("https://github.com")
                                    .set_children(vec![
                                        typography_create(Span).set_text("Visit GitHub").build(),
                                    ])
                                    .build(),
                            ])
                            .build(),
                        hr(),
                        structure_create(StrutureType::Section)
                            .add_class("box")
                            .set_children(vec![
                                typography_create(H2)
                                    .set_text("Scope & Custom Tags")
                                    .build(),
                                scope_create()
                                    .set_children(vec![
                                        typography_create(P)
                                            .set_text("Inside a generic scope (div)")
                                            .build(),
                                    ])
                                    .build(),
                                custom_tag_create("button")
                                    .set_content("This is a custom tag (button)")
                                    .build(),
                            ])
                            .build(),
                        hr(),
                        structure_create(StrutureType::Footer)
                            .set_children(vec![
                                typography_create(P)
                                    .set_text("Footer structure tag")
                                    .build(),
                            ])
                            .build(),
                    ])
                    .build(),
            );

            Html(home.render())
        }),
    }];
    start(&paginas, vec![static_serve("assets", "/assets")]);
}
