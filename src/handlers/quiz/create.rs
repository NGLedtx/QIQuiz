use axum::{
    Form,
    body::Body,
    extract::State,
    http::{Response, StatusCode, header::LOCATION},
};
use sea_orm::{
    ActiveValue::{NotSet, Set},
    ColumnTrait, Condition, EntityTrait, PaginatorTrait, QueryFilter, TransactionTrait,
};
use serde::Deserialize;
use validator::Validate;

use crate::{
    entities::{alternative, category, difficulty, question, quiz},
    jwt::JwtClaims,
    state::AppState,
};

#[derive(Deserialize, Validate)]
pub struct Payload {
    #[validate(range(min = 1))]
    questions: i32,
    id_difficult: i32,
    id_category: i32,
}

#[derive(Deserialize)]
struct ApiResponse {
    response_code: u32,
    results: Vec<Question>,
}

#[derive(Deserialize)]
struct Question {
    question: String,
    correct_answer: String,
    incorrect_answers: Vec<String>,
}

pub async fn create(
    State(state): State<AppState>,
    token: JwtClaims,
    form: Form<Payload>,
) -> Response<Body> {
    if !token.is_admin {
        return Response::builder()
            .status(StatusCode::SEE_OTHER)
            .header(LOCATION, "/user/login")
            .body(Body::empty())
            .unwrap();
    }

    if form.validate().is_err() {
        return Response::builder()
            .status(StatusCode::SEE_OTHER)
            .header(LOCATION, "/quiz/list?error=true")
            .body(Body::empty())
            .unwrap();
    }

    let db = &*state.db_conn;
    let http_client = &*state.http_client;

    let Ok(quiz_exists) = quiz::Entity::find()
        .filter(
            Condition::all()
                .add(quiz::Column::IdDifficulty.eq(form.id_difficult))
                .add(quiz::Column::IdCategory.eq(form.id_category)),
        )
        .count(db)
        .await
    else {
        return Response::builder()
            .status(StatusCode::SEE_OTHER)
            .header(LOCATION, "/quiz/list?error=true")
            .body(Body::empty())
            .unwrap();
    };

    if quiz_exists != 0 {
        return Response::builder()
            .status(StatusCode::SEE_OTHER)
            .header(LOCATION, "/quiz/list?error=true")
            .body(Body::empty())
            .unwrap();
    }

    let Ok(difficult_exists) = difficulty::Entity::find_by_id(form.id_difficult)
        .one(db)
        .await
    else {
        return Response::builder()
            .status(StatusCode::SEE_OTHER)
            .header(LOCATION, "/quiz/list?error=true")
            .body(Body::empty())
            .unwrap();
    };

    let Some(difficult) = difficult_exists else {
        return Response::builder()
            .status(StatusCode::SEE_OTHER)
            .header(LOCATION, "/quiz/list?error=true")
            .body(Body::empty())
            .unwrap();
    };

    let Ok(category_exists) = category::Entity::find_by_id(form.id_category)
        .count(db)
        .await
    else {
        return Response::builder()
            .status(StatusCode::SEE_OTHER)
            .header(LOCATION, "/quiz/list?error=true")
            .body(Body::empty())
            .unwrap();
    };

    if category_exists != 1 {
        return Response::builder()
            .status(StatusCode::SEE_OTHER)
            .header(LOCATION, "/quiz/list?error=true")
            .body(Body::empty())
            .unwrap();
    }

    let Ok(questions) = http_client
        .get(format!(
            "https://opentdb.com/api.php?amount={}&category={}&difficulty={}&type=multiple",
            form.questions, form.id_category, difficult.name
        ))
        .send()
        .await
    else {
        return Response::builder()
            .status(StatusCode::SEE_OTHER)
            .header(LOCATION, "/quiz/list?error=true")
            .body(Body::empty())
            .unwrap();
    };

    let Ok(data) = questions.json::<ApiResponse>().await else {
        return Response::builder()
            .status(StatusCode::SEE_OTHER)
            .header(LOCATION, "/quiz/list?error=true")
            .body(Body::empty())
            .unwrap();
    };

    if data.response_code != 0 {
        return Response::builder()
            .status(StatusCode::SEE_OTHER)
            .header(LOCATION, "/quiz/list?error=true")
            .body(Body::empty())
            .unwrap();
    }

    let questions = data.results;

    let Ok(txn) = db.begin().await else {
        return Response::builder()
            .status(StatusCode::SEE_OTHER)
            .header(LOCATION, "/quiz/list?error=true")
            .body(Body::empty())
            .unwrap();
    };

    let quiz = quiz::ActiveModel {
        id: NotSet,
        questions: Set(form.questions),
        id_category: Set(form.id_category),
        id_difficulty: Set(form.id_difficult),
    };

    let Ok(quiz_result) = quiz::Entity::insert(quiz).exec(&txn).await else {
        return Response::builder()
            .status(StatusCode::SEE_OTHER)
            .header(LOCATION, "/quiz/list?error=true")
            .body(Body::empty())
            .unwrap();
    };

    let question_models = questions
        .iter()
        .map(|question| {
            return question::ActiveModel {
                id: NotSet,
                text: Set(question.question.clone()),
                id_quiz: Set(quiz_result.last_insert_id),
            };
        })
        .collect::<Vec<question::ActiveModel>>();

    let Ok(question_result) = question::Entity::insert_many(question_models)
        .exec(&txn)
        .await
    else {
        let _ = txn.rollback().await;

        return Response::builder()
            .status(StatusCode::SEE_OTHER)
            .header(LOCATION, "/quiz/list?error=true")
            .body(Body::empty())
            .unwrap();
    };

    let mut alternative_models =
        Vec::<alternative::ActiveModel>::with_capacity(form.questions as usize * 4);

    for (index, question) in questions.into_iter().enumerate().map(|(i, q)| (i + 1, q)) {
        let id_question = question_result.last_insert_id - form.questions + index as i32;

        alternative_models.push(alternative::ActiveModel {
            id: NotSet,
            text: Set(question.correct_answer),
            correct: Set(true),
            id_question: Set(id_question),
        });

        question.incorrect_answers.into_iter().for_each(|answer| {
            alternative_models.push(alternative::ActiveModel {
                id: NotSet,
                text: Set(answer),
                correct: Set(false),
                id_question: Set(id_question),
            });
        });
    }

    let Ok(_) = alternative::Entity::insert_many(alternative_models)
        .exec(&txn)
        .await
    else {
        let _ = txn.rollback().await;

        return Response::builder()
            .status(StatusCode::SEE_OTHER)
            .header(LOCATION, "/quiz/list?error=true")
            .body(Body::empty())
            .unwrap();
    };

    if txn.commit().await.is_err() {
        return Response::builder()
            .status(StatusCode::SEE_OTHER)
            .header(LOCATION, "/quiz/list?error=true")
            .body(Body::empty())
            .unwrap();
    };

    Response::builder()
        .status(StatusCode::SEE_OTHER)
        .header(LOCATION, "/quiz/list?error=false")
        .body(Body::empty())
        .unwrap()
}
