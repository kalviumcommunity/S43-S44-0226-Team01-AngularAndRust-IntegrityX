use axum::{routing::get, Router};
use std::net::SocketAddr;

async fn root() ->&'static str {
    "IntegrityX API Running 🚀"
}

#[tokio::main]
async fn main(){
    let app = Router::new().route("/",get(root));
    let addr = SocketAddr::from(([127,0,0,1],3000));
    println!("Server is running on {}",addr);

    let listner = tokio::net::TcpListener::bind(addr)
    .await
    .unwrap();
    axum::serve(listner, app)
    .await
    .unwrap();
}