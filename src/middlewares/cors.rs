use tower_http::cors::CorsLayer;

use crate::configs;

pub fn get_cors() -> CorsLayer {
    let cors_config = configs::get_cors_config();

    CorsLayer::new()
        .allow_origin(cors_config.origins.clone())
        .allow_methods(cors_config.methods.clone())
        .allow_headers(cors_config.headers.clone())
        .allow_credentials(true)
}