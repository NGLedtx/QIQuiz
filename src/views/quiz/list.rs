use askama::Template;
use axum::{
    Form,
    body::Body,
    http::{Response, header::LOCATION},
    response::Html,
};
use reqwest::StatusCode;
use serde::Deserialize;

use crate::jwt::JwtClaims;

#[derive(Template)]
#[template(path = "quiz/list.html")]
struct MyTemplate {
    error: Option<bool>,
}

#[derive(Deserialize)]
pub struct Payload {
    error: Option<bool>,
}

pub async fn list(token: JwtClaims, form: Form<Payload>) -> Result<Html<String>, Response<Body>> {
    if !token.is_admin {
        return Err(Response::builder()
            .status(StatusCode::SEE_OTHER)
            .header(LOCATION, "/user/login")
            .body(Body::empty())
            .unwrap());
    }

    let template = MyTemplate {
        error: form.error.clone(),
    }
    .render()
    .unwrap();

    Ok(Html(template))
}
