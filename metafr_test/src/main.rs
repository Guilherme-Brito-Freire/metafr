use metafr::components::typography::TypographyType::{H1, P};
use metafr::components::{
    typography,
    scope
};
use metafr::document::{Document, create_document};
use metafr::params::style_inline::StyleTag::Color;
use metafr::params::style_inline::get_style_inline;
use metafr::{Html, Page, get, start, static_serve};

fn main() {

    let paginas = vec![
        Page {
            path: "/".to_string(),
            method: get(|| async { 
                // Home page!
                let home: Document = create_document(
                    "<link rel='stylesheet' href='static/style.css'>", // Header
                    scope::scope_create()
                    .set_children(
                    vec![
                        typography::typography_create(H1)
                        .set_text("Hello World!")
                        .build(),

                        typography::typography_create(P)
                        .set_text("Hello Guilherme!")
                        .set_params(vec![get_style_inline(vec![Color.get_tag("red")])])
                        .build()
                    ]
                )
                    .build()
                );

                Html(home.render())
            }),
        }
    ];
    
    // Start to serve statics files!
    let static_server = static_serve("./static","/static");

    start(&paginas, vec![static_server]);

}