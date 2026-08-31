use std::convert::Infallible;

pub use axum::{Router, routing::{ MethodRouter, get, post, delete} };
pub use axum::response::Html;

use tokio::{net::TcpListener}; 

pub struct Page {
    pub path:String,
    pub method: MethodRouter<(),Infallible>
}

#[tokio::main]
pub async fn start(pages: &[Page] ) {
    let mut app: Router<()> = Router::new();

    for page_item  in pages {
        app = app.route(&*page_item.path, page_item.method.clone() );
    }

    let listener: TcpListener =  TcpListener::bind("127.0.0.1:8080").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

pub mod document;
pub mod ast;
pub mod components;
pub mod param;
pub mod params; // Collection