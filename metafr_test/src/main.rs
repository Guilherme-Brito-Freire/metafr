use metafr::{Page, document::{Document, create_document}, get, start };
use metafr::Html;

// Retorne Html(...) em vez de string pura
pub async fn meu_handler() -> Html<&'static str> {
    Html("<!DOCTYPE html><html><body><h1>Hello World!</h1></body></html>")
}

fn main() {

    let paginas = vec![
        Page {
            path: "/".to_string(),
            method: get(|| async { 
                // Home page!
                let home: Document = create_document(); 
                Html(home.render())
            }),

        },
        Page {
            path: "/sobre".to_string(),
            method: get(|| async { "Sobre a Empresa" }),
        }
    ];

    start(&paginas);

}