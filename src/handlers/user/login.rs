use axum::{
    Form,
    body::Body,
    http::header::{LOCATION, SET_COOKIE},
    response::Response,
};
use axum_extra::extract::cookie::Cookie;
use reqwest::StatusCode;
use serde::Deserialize;

use crate::{configs, jwt::JwtClaims};

#[derive(Deserialize)]
pub struct Payload {
    user: String,
    password: String,
}

pub async fn login(form: Form<Payload>) -> Response {
    let app_config = configs::get_app_config();

    if form.user != app_config.user_admin || form.password != app_config.user_password {
        return Response::builder()
            .status(StatusCode::SEE_OTHER)
            .header(LOCATION, "/user/login?error=true")
            .body(Body::empty())
            .unwrap();
    }

    let token = JwtClaims::new(true, None).gen_token();

    let cookie = Cookie::build(("token", token)).http_only(true).path("/");

    Response::builder()
        .status(StatusCode::SEE_OTHER)
        .header(SET_COOKIE, cookie.to_string())
        .header(LOCATION, "/quiz/list")
        .body(Body::empty())
        .unwrap()
}
