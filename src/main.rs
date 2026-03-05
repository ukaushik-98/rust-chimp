use anyhow::Result;
use axum::{Router, extract::Path, routing::get};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> Result<()> {
    let app = Router::new()
        .route("/", get(root))
        .route("/greet/{name}", get(greet));
    let listener = TcpListener::bind("0.0.0.0:3000").await?;
    let _ = axum::serve(listener, app).await;
    Ok(())
}

async fn root() -> &'static str {
    "Hello World!"
}

async fn greet(Path(name): Path<String>) -> String {
    format!("Hello {}", name)
}
