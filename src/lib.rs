use crate::routes::health_check::health_check;
use anyhow::Result;
use axum::{Router, extract::Path, routing::get};
use tokio::net::TcpListener;

mod routes;

pub async fn run() -> Result<()> {
    let app = Router::new()
        .route("/", get(root))
        .route("/greet/{name}", get(greet))
        .route("/health_check", get(health_check));
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

#[cfg(test)]
mod tests {
    use super::*;
    use reqwest::Client;

    #[tokio::test]
    async fn health_check_test() {
        let app = spawn_app().await.expect("app must be booted");

        let response = Client::new()
            .get("http://127.0.0.1:8080/health_check")
            .send()
            .await
            .expect("failed to send request");
    }

    async fn spawn_app() -> Result<()> {
        todo!()
    }
}
