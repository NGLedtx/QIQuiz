use axum::{
    body::Body,
    http::{
        StatusCode,
        header::{LOCATION, SET_COOKIE},
    },
    response::Response,
};
use axum_extra::extract::cookie::Cookie;

pub async fn logoff() -> Response {
    let cookie = Cookie::build(("token", "")).path("/").http_only(true);

    Response::builder()
        .status(StatusCode::SEE_OTHER)
        .header(SET_COOKIE, cookie.to_string())
        .header(LOCATION, "/quiz/init")
        .body(Body::empty())
        .unwrap()
}
