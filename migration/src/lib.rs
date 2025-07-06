pub use sea_orm_migration::prelude::*;

mod m20250706_214212_create_table_rank;
mod m20250706_221057_create_table_category;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20250706_214212_create_table_rank::Migration),
            Box::new(m20250706_221057_create_table_category::Migration),
        ]
    }
}
