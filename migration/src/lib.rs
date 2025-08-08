pub use sea_orm_migration::prelude::*;

mod m20250808_000440_create_rank_table;
mod m20250808_000448_create_quiz_table;
mod m20250808_000506_create_category_table;
mod m20250808_000513_create_difficult_table;
mod m20250808_000525_create_question_table;
mod m20250808_003742_create_alternative_table;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20250808_000513_create_difficult_table::Migration),
            Box::new(m20250808_000506_create_category_table::Migration),
            Box::new(m20250808_000448_create_quiz_table::Migration),
            Box::new(m20250808_000440_create_rank_table::Migration),
            Box::new(m20250808_000525_create_question_table::Migration),
            Box::new(m20250808_003742_create_alternative_table::Migration),
        ]
    }
}
