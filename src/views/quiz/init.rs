use askama::Template;
use axum::{extract::State, response::Html};
use sea_orm::{
    EntityTrait, FromQueryResult, JoinType, Order, QueryOrder, QuerySelect, RelationTrait,
};

use crate::{
    entities::{category, difficulty, quiz},
    state::AppState,
};

#[derive(Template)]
#[template(path = "quiz/init.html")]
struct MyTemplate {
    quizes: Vec<Quiz>,
}

#[derive(FromQueryResult)]
struct Quiz {
    id: i32,
    questions: i32,
    difficulty: String,
    category: String,
}

pub async fn init(State(state): State<AppState>) -> Html<String> {
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
        let template = MyTemplate { quizes: vec![] }.render().unwrap();

        return Html(template);
    };

    let template = MyTemplate { quizes }.render().unwrap();

    Html(template)
}
