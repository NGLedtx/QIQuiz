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
                    .table(Rank::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Rank::Id)
                            .integer()
                            .auto_increment()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Rank::Name).string_len(30).not_null())
                    .col(ColumnDef::new(Rank::Time).time().not_null())
                    .col(
                        ColumnDef::new(Rank::Questions)
                            .integer()
                            .not_null()
                            .check(Expr::col(Rank::Questions).gte(0)),
                    )
                    .col(ColumnDef::new(Rank::IdQuiz).integer().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-rank-quiz-id")
                            .from(Rank::Table, Rank::IdQuiz)
                            .to(Quiz::Table, Quiz::Id),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Rank::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Rank {
    Table,
    Id,
    Name,
    Time,
    Questions,
    IdQuiz,
}
