use askama::Template;
use axum::{Form, response::Html};
use serde::Deserialize;

#[derive(Template)]
#[template(path = "user/login.html")]
struct MyTemplate {
    error: Option<String>,
}

#[derive(Deserialize)]
pub struct Payload {
    error: Option<String>,
}

pub async fn login(form: Form<Payload>) -> Html<String> {
    let template = MyTemplate {
        error: form.error.clone(),
    }
    .render()
    .unwrap();

    Html(template)
}
