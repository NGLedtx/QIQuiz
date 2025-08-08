use sea_orm_migration::prelude::*;

use crate::m20250808_000525_create_question_table::Question;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Alternative::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Alternative::Id)
                            .integer()
                            .auto_increment()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Alternative::Text).string().not_null())
                    .col(ColumnDef::new(Alternative::Correct).boolean().not_null())
                    .col(ColumnDef::new(Alternative::IdQuestion).integer().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-alternative-question-id")
                            .from(Alternative::Table, Alternative::IdQuestion)
                            .to(Question::Table, Question::Id),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Alternative::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Alternative {
    Table,
    Id,
    Text,
    Correct,
    IdQuestion,
}
