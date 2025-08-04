use std::sync::Arc;

use sea_orm::DatabaseConnection;

#[derive(Clone)]
pub struct AppState {
    pub db_conn: Arc<DatabaseConnection>,
}
