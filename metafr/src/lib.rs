use std::convert::Infallible;

pub use axum::{Router, routing::{ MethodRouter, get, post, delete} };
pub use axum::response::Html;
use tower_http::services::ServeDir;

use tokio::{net::TcpListener}; 

pub struct Page {
    pub path:String,
    pub method: MethodRouter<(),Infallible>
}

pub struct StaticServer{
    serve_dir: ServeDir,
    endpoint: String
}

pub fn static_serve(path: &str, endpoint: &str) -> StaticServer {
    StaticServer{
        serve_dir: ServeDir::new(path),
        endpoint: endpoint.to_string()
    }
}
#[tokio::main]
pub async fn start(pages: &[Page], statics: Vec<StaticServer> ) {
    let mut app: Router<()> = Router::new();

    for page_item in pages {
        app = app.route(&*page_item.path, page_item.method.clone() );
    }

    for item in statics {
        app = app.nest_service(&item.endpoint, item.serve_dir)
    }

    let listener: TcpListener =  TcpListener::bind("127.0.0.1:8080").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

pub mod document;
pub mod ast;
pub mod components;
pub mod param;
pub mod params; // Collection