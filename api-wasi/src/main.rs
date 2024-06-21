// mod repo;
// mod services;
// use anyhow::Result;
// use axum::Server;
// use std::net::SocketAddr;

// #[tokio::main(flavor = "multi_thread", worker_threads = 10)]
// async fn main() -> Result<()> {
//     run_main().await
// }

// async fn run_main() -> Result<()> {
//     println!("starting server...");
//     let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
//     axum::Server::bind(&addr)
//         .serve(app.into_make_service())
//         .await
//         .unwrap();
//     let router = crate::services::router().await;
//     let server = Server::bind(&addr).serve(router.into_make_service());

//     print!("server running on {}", addr);
//     if let Err(err) = server.await {
//         print!("error while serving: {}", err);
//     }

//     Ok(())
// }

use axum::{routing::get, Router};
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    // Building our application with a single Route
    let app = Router::new().route("/", get(handler));
 
    // Run the server with hyper on http://127.0.0.1:3000
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
 
async fn handler() -> &'static str {
    "Hello, Axum ❤️ WASIX!"
}
 