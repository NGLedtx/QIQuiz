mod database;

pub async fn init_connections() {
    database::init_database_connection().await;
}