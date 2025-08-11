use askama::Template;
use axum::{
    extract::{Path, State},
    response::Html,
};

use migration::Expr;
use sea_orm::{
    ColumnTrait, EntityTrait, FromQueryResult, QueryFilter, QuerySelect, entity::prelude::Time,
};

use crate::{entities::rank, state::AppState};

#[derive(Template)]
#[template(path = "quiz/rank.html")]
struct MyTemplate {
    ranks: Vec<Rank>,
}

#[derive(FromQueryResult)]
struct Rank {
    position: i32,
    name: String,
    time: Time,
    questions: i32,
}

pub async fn rank(State(state): State<AppState>, Path(quiz_id): Path<i32>) -> Html<String> {
    let db = &*state.db_conn;

    let Ok(ranks) = rank::Entity::find()
        .filter(rank::Column::IdQuiz.eq(quiz_id))
        .select_only()
        .columns([rank::Column::Name, rank::Column::Time, rank::Column::Questions])
        .column_as(Expr::cust("ROW_NUMBER() over ( PARTITION BY rank.id_quiz ORDER BY rank.questions DESC, rank.time)"), "position")
        .into_model::<Rank>()
        .all(db)
        .await
    else {
        let template = MyTemplate { ranks: vec![] }.render().unwrap();

        return Html(template);
    };

    let template = MyTemplate { ranks }.render().unwrap();

    Html(template)
}
