use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Difficult::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Difficult::Id)
                            .integer()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Difficult::Name).string().not_null())
                    .to_owned(),
            )
            .await?;

        manager
            .exec_stmt(
                Query::insert()
                    .into_table(Difficult::Table)
                    .columns([Difficult::Id, Difficult::Name])
                    .values_panic([1.into(), "easy".into()])
                    .values_panic([2.into(), "medium".into()])
                    .values_panic([3.into(), "hard".into()])
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Difficult::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum Difficult {
    Table,
    Id,
    Name,
}
