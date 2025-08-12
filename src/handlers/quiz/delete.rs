use axum::{
    body::Body,
    extract::{Path, State},
    http::{StatusCode, header::LOCATION},
    response::Response,
};
use migration::{Expr, Query};
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter, TransactionTrait};

use crate::{
    entities::{alternative, question, quiz, rank},
    jwt::JwtClaims,
    state::AppState,
};

pub async fn delete(
    State(state): State<AppState>,
    token: JwtClaims,
    Path(quiz_id): Path<i32>,
) -> Response {
    if !token.is_admin {
        return Response::builder()
            .status(StatusCode::SEE_OTHER)
            .header(LOCATION, "/user/login")
            .body(Body::empty())
            .unwrap();
    }

    let db = &*state.db_conn;

    let Ok(txn) = db.begin().await else {
        return Response::builder()
            .status(StatusCode::SEE_OTHER)
            .header(LOCATION, "/quiz/list?error=true")
            .body(Body::empty())
            .unwrap();
    };

    if rank::Entity::delete_many()
        .filter(rank::Column::IdQuiz.eq(quiz_id))
        .exec(&txn)
        .await
        .is_err()
    {
        return Response::builder()
            .status(StatusCode::SEE_OTHER)
            .header(LOCATION, "/quiz/list?error=true")
            .body(Body::empty())
            .unwrap();
    }

    if alternative::Entity::delete_many()
        .filter(Expr::exists(
            Query::select()
                .expr(1)
                .from(question::Entity)
                .and_where(
                    Expr::col(question::Column::Id)
                        .equals(alternative::Column::IdQuestion)
                        .and(Expr::col(question::Column::IdQuiz).eq(quiz_id)),
                )
                .to_owned(),
        ))
        .exec(&txn)
        .await
        .is_err()
    {
        let _ = txn.rollback().await;

        return Response::builder()
            .status(StatusCode::SEE_OTHER)
            .header(LOCATION, "/quiz/list?error=true")
            .body(Body::empty())
            .unwrap();
    }

    if question::Entity::delete_many()
        .filter(question::Column::IdQuiz.eq(quiz_id))
        .exec(&txn)
        .await
        .is_err()
    {
        let _ = txn.rollback().await;

        return Response::builder()
            .status(StatusCode::SEE_OTHER)
            .header(LOCATION, "/quiz/list?error=true")
            .body(Body::empty())
            .unwrap();
    }

    if quiz::Entity::delete_by_id(quiz_id)
        .exec(&txn)
        .await
        .is_err()
    {
        let _ = txn.rollback().await;

        return Response::builder()
            .status(StatusCode::SEE_OTHER)
            .header(LOCATION, "/quiz/list?error=true")
            .body(Body::empty())
            .unwrap();
    }

    if txn.commit().await.is_err() {
        return Response::builder()
            .status(StatusCode::SEE_OTHER)
            .header(LOCATION, "/quiz/list?error=true")
            .body(Body::empty())
            .unwrap();
    };

    Response::builder()
        .status(StatusCode::SEE_OTHER)
        .header(LOCATION, "/quiz/list")
        .body(Body::empty())
        .unwrap()
}
