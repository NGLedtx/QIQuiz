use askama::Template;
use axum::{
    Form,
    body::Body,
    extract::State,
    http::{Response, header::LOCATION},
    response::Html,
};
use reqwest::StatusCode;
use sea_orm::{
    EntityTrait, FromQueryResult, JoinType, Order, QueryOrder, QuerySelect, RelationTrait,
};
use serde::Deserialize;

use crate::{
    entities::{category, difficulty, quiz},
    jwt::JwtClaims,
    state::AppState,
};

#[derive(Deserialize)]
pub struct Payload {
    error: Option<bool>,
}

#[derive(Template)]
#[template(path = "quiz/list.html")]
struct MyTemplate {
    error: Option<bool>,
    quizes: Vec<Quiz>,
}

#[derive(FromQueryResult)]
struct Quiz {
    id: i32,
    questions: i32,
    difficulty: String,
    category: String,
}

pub async fn list(
    State(state): State<AppState>,
    token: JwtClaims,
    form: Form<Payload>,
) -> Result<Html<String>, Response<Body>> {
    if !token.is_admin {
        return Err(Response::builder()
            .status(StatusCode::SEE_OTHER)
            .header(LOCATION, "/user/login")
            .body(Body::empty())
            .unwrap());
    }

    let db = &*state.db_conn;

    let Ok(quizes) = quiz::Entity::find()
        .columns([quiz::Column::Id, quiz::Column::Questions])
        .column_as(difficulty::Column::Name, "difficulty")
        .column_as(category::Column::Name, "category")
        .join(JoinType::LeftJoin, quiz::Relation::Difficulty.def())
        .join(JoinType::LeftJoin, quiz::Relation::Category.def())
        .order_by(category::Column::Name, Order::Asc)
        .order_by(difficulty::Column::Id, Order::Asc)
        .order_by(quiz::Column::Questions, Order::Asc)
        .into_model::<Quiz>()
        .all(db)
        .await
    else {
        let template = MyTemplate {
            error: form.error.clone(),
            quizes: vec![],
        }
        .render()
        .unwrap();

        return Ok(Html(template));
    };

    let template = MyTemplate {
        error: form.error.clone(),
        quizes,
    }
    .render()
    .unwrap();

    Ok(Html(template))
}
