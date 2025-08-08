use sea_orm_migration::prelude::*;

use crate::{
    m20250808_000506_create_category_table::Category,
    m20250808_000513_create_difficulty_table::Difficulty,
};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Quiz::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Quiz::Id)
                            .integer()
                            .auto_increment()
                            .not_null()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(Quiz::Questions)
                            .integer()
                            .not_null()
                            .check(Expr::col(Quiz::Questions).gt(0)),
                    )
                    .col(ColumnDef::new(Quiz::IdCategory).integer().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-quiz-category-id")
                            .from(Quiz::Table, Quiz::IdCategory)
                            .to(Category::Table, Category::Id),
                    )
                    .col(ColumnDef::new(Quiz::IdDifficulty).integer().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-quiz-difficulty-id")
                            .from(Quiz::Table, Quiz::IdDifficulty)
                            .to(Difficulty::Table, Difficulty::Id),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Quiz::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum Quiz {
    Table,
    Id,
    Questions,
    IdCategory,
    IdDifficulty,
}
