use sea_orm_migration::prelude::*;

use crate::m20250808_000448_create_quiz_table::Quiz;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Question::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Question::Id)
                            .integer()
                            .auto_increment()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Question::Text).string().not_null())
                    .col(ColumnDef::new(Question::IdQuiz).integer().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-question-quiz-id")
                            .from(Question::Table, Question::IdQuiz)
                            .to(Quiz::Table, Quiz::Id),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Question::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum Question {
    Table,
    Id,
    Text,
    IdQuiz,
}
