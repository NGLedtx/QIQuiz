use sea_orm_migration::prelude::*;

use crate::{
    m20250706_214212_create_table_difficulty::Difficulty,
    m20250706_221057_create_table_category::Category,
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
                    .col(ColumnDef::new(Quiz::Uuid).uuid().primary_key())
                    .col(ColumnDef::new(Quiz::Questions).integer().not_null())
                    .col(ColumnDef::new(Quiz::IdDifficulty).integer().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-quiz-difficulty-id")
                            .from(Quiz::Table, Quiz::IdDifficulty)
                            .to(Difficulty::Table, Difficulty::Id),
                    )
                    .col(ColumnDef::new(Quiz::IdCategory).integer().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-quiz-category-id")
                            .from(Quiz::Table, Quiz::IdCategory)
                            .to(Category::Table, Category::Id),
                    )
                    .to_owned(),
            )
            .await?;

        Ok(())
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
    Uuid,
    Questions,
    IdDifficulty,
    IdCategory,
}
