use tokio::net::TcpListener;

mod app;

#[tokio::main]
async fn main() {
    let app = app::create_app().await;

    let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();

    axum::serve(listener, app).await.unwrap();
}
