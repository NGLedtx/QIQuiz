use std::{
    collections::HashMap,
    time::{Duration, SystemTime, UNIX_EPOCH},
};

use axum::{
    Form,
    body::Body,
    extract::State,
    http::{StatusCode, header::LOCATION},
    response::Response,
};
use chrono::NaiveTime;
use sea_orm::{
    ActiveValue::{NotSet, Set},
    ColumnTrait, Condition, EntityTrait, FromQueryResult, JoinType, Order, QueryFilter, QueryOrder,
    QuerySelect, RelationTrait,
};

use crate::{
    entities::{alternative, question, rank},
    jwt::JwtClaims,
    state::AppState,
};

#[derive(FromQueryResult)]
struct Question {
    question: i32,
    alternative: i32,
}

pub async fn submit(
    State(state): State<AppState>,
    token: JwtClaims,
    form: Form<HashMap<String, String>>,
) -> Response<Body> {
    let Some(name) = form.get("name") else {
        return Response::builder()
            .status(StatusCode::SEE_OTHER)
            .header(LOCATION, "/quiz/init?error=true")
            .body(Body::empty())
            .unwrap();
    };

    let db = &*state.db_conn;

    let quiz_id = token.quiz_id.unwrap();

    let Ok(questions) = question::Entity::find()
        .select_only()
        .column_as(question::Column::Id, "question")
        .column_as(alternative::Column::Id, "alternative")
        .filter(
            Condition::all()
                .add(question::Column::IdQuiz.eq(quiz_id))
                .add(alternative::Column::Correct.eq(true)),
        )
        .join(JoinType::LeftJoin, question::Relation::Alternative.def())
        .into_model::<Question>()
        .all(db)
        .await
    else {
        return Response::builder()
            .status(StatusCode::SEE_OTHER)
            .header(LOCATION, "/quiz/init?error=true")
            .body(Body::empty())
            .unwrap();
    };

    let mut score = 0;

    for question in questions {
        let Some(resp) = form.get(&question.question.to_string()) else {
            return Response::builder()
                .status(StatusCode::SEE_OTHER)
                .header(LOCATION, "/quiz/init?error=true")
                .body(Body::empty())
                .unwrap();
        };

        if question.alternative.to_string() == *resp {
            score += 1;
        }
    }

    let Ok(min_rank) = rank::Entity::find()
        .filter(rank::Column::IdQuiz.eq(quiz_id))
        .order_by(rank::Column::Questions, Order::Asc)
        .order_by(rank::Column::Time, Order::Desc)
        .limit(1)
        .one(db)
        .await
    else {
        return Response::builder()
            .status(StatusCode::SEE_OTHER)
            .header(LOCATION, "/quiz/init?error=true")
            .body(Body::empty())
            .unwrap();
    };

    if min_rank.is_none() {
        let now = SystemTime::now();

        let created_at_time = UNIX_EPOCH + Duration::from_secs(token.created_at as u64);

        let diff = now.duration_since(created_at_time).unwrap(); // Duration

        let total_secs = diff.as_secs();
        let hours = total_secs / 3600;
        let minutes = (total_secs % 3600) / 60;
        let seconds = total_secs % 60;

        let rank = rank::ActiveModel {
            id: NotSet,
            id_quiz: Set(quiz_id),
            name: Set(name.to_owned()),
            questions: Set(score),
            time: Set(
                NaiveTime::from_hms_opt(hours as u32, minutes as u32, seconds as u32).unwrap(),
            ),
        };

        if rank::Entity::insert(rank).exec(db).await.is_err() {
            return Response::builder()
                .status(StatusCode::SEE_OTHER)
                .header(LOCATION, "/quiz/init?error=true")
                .body(Body::empty())
                .unwrap();
        }
    } else {
        let min_rank = min_rank.unwrap();

        if min_rank.questions > score {
            return Response::builder()
                .status(StatusCode::SEE_OTHER)
                .header(LOCATION, "/quiz/init")
                .body(Body::empty())
                .unwrap();
        } else if min_rank.questions < score {
            let now = SystemTime::now();

            let created_at_time = UNIX_EPOCH + Duration::from_secs(token.created_at as u64);

            let diff = now.duration_since(created_at_time).unwrap(); // Duration

            let total_secs = diff.as_secs();
            let hours = total_secs / 3600;
            let minutes = (total_secs % 3600) / 60;
            let seconds = total_secs % 60;

            let rank = rank::ActiveModel {
                id: NotSet,
                id_quiz: Set(quiz_id),
                name: Set(name.to_owned()),
                questions: Set(score),
                time: Set(
                    NaiveTime::from_hms_opt(hours as u32, minutes as u32, seconds as u32).unwrap(),
                ),
            };

            if rank::Entity::insert(rank).exec(db).await.is_err() {
                return Response::builder()
                    .status(StatusCode::SEE_OTHER)
                    .header(LOCATION, "/quiz/init?error=true")
                    .body(Body::empty())
                    .unwrap();
            }

            if rank::Entity::delete_by_id(min_rank.id)
                .exec(db)
                .await
                .is_err()
            {
                return Response::builder()
                    .status(StatusCode::SEE_OTHER)
                    .header(LOCATION, "/quiz/init?error=true")
                    .body(Body::empty())
                    .unwrap();
            }
        }
    }

    return Response::builder()
        .status(StatusCode::SEE_OTHER)
        .header(LOCATION, format!("/quiz/{}/rank", quiz_id))
        .body(Body::empty())
        .unwrap();
}
