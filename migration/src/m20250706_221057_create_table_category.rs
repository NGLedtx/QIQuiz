use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Category::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Category::Id).integer().primary_key())
                    .col(ColumnDef::new(Category::Name).string().not_null())
                    .to_owned(),
            )
            .await?;

        manager
            .exec_stmt(
                Query::insert()
                    .into_table(Category::Table)
                    .columns([Category::Id, Category::Name])
                    .values_panic([9.into(), "General Knowledge".into()])
                    .values_panic([10.into(), "Entertainment: Books".into()])
                    .values_panic([11.into(), "Entertainment: Film".into()])
                    .values_panic([12.into(), "Entertainment: Music".into()])
                    .values_panic([13.into(), "Entertainment: Musicals & Theatres".into()])
                    .values_panic([14.into(), "Entertainment: Television".into()])
                    .values_panic([15.into(), "Entertainment: Video Games".into()])
                    .values_panic([16.into(), "Entertainment: Board Games".into()])
                    .values_panic([17.into(), "Science & Nature".into()])
                    .values_panic([18.into(), "Science: Computers".into()])
                    .values_panic([19.into(), "Science: Mathematics".into()])
                    .values_panic([20.into(), "Mythology".into()])
                    .values_panic([21.into(), "Sports".into()])
                    .values_panic([22.into(), "Geography".into()])
                    .values_panic([23.into(), "History".into()])
                    .values_panic([24.into(), "Politics".into()])
                    .values_panic([25.into(), "Art".into()])
                    .values_panic([26.into(), "Celebrities".into()])
                    .values_panic([27.into(), "Animals".into()])
                    .values_panic([28.into(), "Vehicles".into()])
                    .values_panic([29.into(), "Entertainment: Comics".into()])
                    .values_panic([30.into(), "Science: Gadgets".into()])
                    .values_panic([31.into(), "Entertainment: Japanese Anime & Manga".into()])
                    .values_panic([32.into(), "Entertainment: Cartoon & Animations".into()])
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Category::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum Category {
    Table,
    Id,
    Name,
}
