use askama::Template;
use axum::{
    body::Body,
    extract::State,
    http::{Response, header::LOCATION},
    response::Html,
};
use reqwest::StatusCode;
use sea_orm::EntityTrait;

use crate::{
    entities::{category, difficulty},
    jwt::JwtClaims,
    state::AppState,
};

#[derive(Template)]
#[template(path = "quiz/create.html")]
struct MyTemplate {
    difficults: Vec<difficulty::Model>,
    categories: Vec<category::Model>,
}

pub async fn create(
    State(state): State<AppState>,
    token: JwtClaims,
) -> Result<Html<String>, Response<Body>> {
    if !token.is_admin {
        return Err(Response::builder()
            .status(StatusCode::SEE_OTHER)
            .header(LOCATION, "/user/login")
            .body(Body::empty())
            .unwrap());
    }

    let db = &*state.db_conn;

    let Ok(difficults) = difficulty::Entity::find().all(db).await else {
        let template = MyTemplate {
            difficults: vec![],
            categories: vec![],
        }
        .render()
        .unwrap();

        return Ok(Html(template));
    };

    let Ok(categories) = category::Entity::find().all(db).await else {
        let template = MyTemplate {
            difficults: vec![],
            categories: vec![],
        }
        .render()
        .unwrap();

        return Ok(Html(template));
    };

    let template = MyTemplate {
        difficults,
        categories,
    }
    .render()
    .unwrap();

    Ok(Html(template))
}
