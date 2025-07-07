use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Difficulty::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Difficulty::Id)
                            .integer()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Difficulty::Name).string().not_null())
                    .to_owned(),
            )
            .await?;

        manager
            .exec_stmt(
                Query::insert()
                    .into_table(Difficulty::Table)
                    .columns([Difficulty::Name])
                    .values_panic(["easy".into()])
                    .values_panic(["medium".into()])
                    .values_panic(["hard".into()])
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Difficulty::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum Difficulty {
    Table,
    Id,
    Name,
}
