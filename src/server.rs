use axum::Router;

pub async fn serve(app: Router, port: u16) {
    let addr = format!("0.0.0.0:{}", port);
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();

    println!("🥀 Listening on localhost:{} ...", port);
    axum::serve(listener, app).await.unwrap();
} 