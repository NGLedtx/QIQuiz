use sea_orm_migration::prelude::*;

use crate::m20250706_214212_create_table_quiz::Quiz;

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
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Question::Title).string().not_null())
                    .col(ColumnDef::new(Question::UuidQuiz).uuid().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-question-quiz-uuid")
                            .from(Question::Table, Question::UuidQuiz)
                            .to(Quiz::Table, Quiz::Uuid),
                    )
                    .to_owned(),
            )
            .await?;

        Ok(())
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
    Title,
    UuidQuiz,
}
