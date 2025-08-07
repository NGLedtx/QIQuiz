use tokio::net::TcpListener;

mod app;
mod configs;
mod connections;
mod entities;
mod handlers;
mod jwt;
mod middlewares;
mod routes;
mod state;
mod views;

#[tokio::main]
async fn main() {
    #[cfg(debug_assertions)]
    dotenvy::dotenv().ok();

    let port = configs::get_app_config().port;

    let app = app::create_app().await;

    let listener = TcpListener::bind(format!("0.0.0.0:{}", port))
        .await
        .unwrap();

    println!("Server listening on port {}", port);

    axum::serve(listener, app).await.unwrap();
}
