use axum::{
    http::{StatusCode, header::LOCATION},
    response::Response,
};

pub async fn home() -> Response {
    Response::builder()
        .status(StatusCode::SEE_OTHER)
        .header(LOCATION, "/quiz/init")
        .body(axum::body::Body::empty())
        .unwrap()
}
