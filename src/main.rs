use axum::{Router, middleware, routing::post};
use webhooks::auth::verify_github_signature;

mod server;
mod kube;
mod webhooks;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .fallback(post(webhooks::handle)
        .layer(middleware::from_fn(verify_github_signature)));

    server::serve(app, 80).await;
}