use sea_orm_migration::prelude::*;

use crate::m20250706_221057_create_table_category::Category;

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
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Rank::Name).string_len(30).not_null())
                    .col(ColumnDef::new(Rank::IdCategory).integer().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-rank-category-id")
                            .from(Rank::Table, Rank::IdCategory)
                            .to(Category::Table, Category::Id),
                    )
                    .col(ColumnDef::new(Rank::Time).integer().not_null())
                    .col(
                        ColumnDef::new(Rank::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
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
    IdCategory,
    Time,
    CreatedAt,
}
