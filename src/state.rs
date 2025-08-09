use std::sync::Arc;

use reqwest::Client as HttpClient;
use sea_orm::DatabaseConnection;

#[derive(Clone)]
pub struct AppState {
    pub db_conn: Arc<DatabaseConnection>,
    pub http_client: Arc<HttpClient>,
}
