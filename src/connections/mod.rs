mod database;
pub use database::get_database_connection;

pub async fn init_connections() {
    database::init_database_connection().await;
}
