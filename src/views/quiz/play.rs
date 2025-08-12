use askama::Template;
use axum::{
    body::Body,
    extract::{Path, State},
    http::{
        StatusCode,
        header::{LOCATION, SET_COOKIE},
    },
    response::Response,
};
use axum_extra::extract::cookie::Cookie;
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};

use crate::{
    entities::{alternative, question},
    jwt::JwtClaims,
    state::AppState,
};

#[derive(Template)]
#[template(path = "quiz/play.html")]
struct MyTemplate {
    questions: Vec<Question>,
}

struct Question {
    id: i32,
    text: String,
    alternatives: Vec<Alternative>,
}

struct Alternative {
    id: i32,
    text: String,
}

pub async fn play(State(state): State<AppState>, Path(quiz_id): Path<i32>) -> Response<Body> {
    let db = &*state.db_conn;

    let Ok(questions_with_alternatives) = question::Entity::find()
        .filter(question::Column::IdQuiz.eq(quiz_id))
        .find_with_related(alternative::Entity)
        .all(db)
        .await
    else {
        return Response::builder()
            .status(StatusCode::SEE_OTHER)
            .header(LOCATION, "/quiz/init")
            .body(Body::empty())
            .unwrap();
    };

    let questions = questions_with_alternatives
        .into_iter()
        .map(|(question, alternatives)| Question {
            id: question.id,
            text: question.text,
            alternatives: alternatives
                .into_iter()
                .map(|alternative| Alternative {
                    id: alternative.id,
                    text: alternative.text,
                })
                .collect(),
        })
        .collect::<Vec<Question>>();

    let token = JwtClaims::new(false, Some(quiz_id)).gen_token();
    let cookie = Cookie::build(("token", token)).http_only(true).path("/");

    let template = MyTemplate { questions }.render().unwrap();

    Response::builder()
        .header(SET_COOKIE, cookie.to_string())
        .body(Body::from(template))
        .unwrap()
}
