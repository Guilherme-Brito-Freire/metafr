use metafr::{Page, document::{Document, create_document}, get, start };
use metafr::components::{
    typography
};
use metafr::Html;

fn main() {

    let paginas = vec![
        Page {
            path: "/".to_string(),
            method: get(|| async { 
                // Home page!
                let home: Document = create_document(
                    typography::typography_create()
                    .set_text("Hello Guilherme")
                    .build()
                );

                Html(home.render())
            }),
        }
    ];

    start(&paginas);

}