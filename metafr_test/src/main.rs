use metafr::components::{
    typography,
    scope
};
use metafr::document::{Document, create_document};
use metafr::params::style_inline::StyleTag::BackgroundColor;
use metafr::params::style_inline::{get_style_inline};
use metafr::{Html, Page, get, start};

fn main() {

    let paginas = vec![
        Page {
            path: "/".to_string(),
            method: get(|| async { 
                // Home page!
                let home: Document = create_document(
                    "", // Header
                    scope::scope_create()
                    .set_params(vec![
                        get_style_inline(vec![
                            BackgroundColor.get_tag("red")
                        ])
                    ])
                    .set_children(
                    vec![
                        typography::typography_create()
                        .set_text("Hello World!")
                        .build(),

                        typography::typography_create()
                        .set_text("Hello Guilherme!")
                        .build()
                    ]
                )
                    .build()
                );

                Html(home.render())
            }),
        }
    ];

    start(&paginas);

}