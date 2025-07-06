use std::{env, sync::OnceLock};

use sea_orm::ConnectOptions;

static DATABASE_CONFIG: OnceLock<ConnectOptions> = OnceLock::new();

pub fn get_database_config() -> &'static ConnectOptions {
    DATABASE_CONFIG.get_or_init(|| {
        let db_url = env::var("DATABASE_URL").expect("DATABASE_URL not found at .env file");
        let db_max_connections = env::var("DATABASE_MAX_CONNECTIONS")
            .expect("DATABASE_MAX_CONNECTIONS not found at .env file")
            .parse::<u32>()
            .expect("DATABASE_MAX_CONNECTIONS must be a number");
        let db_min_connections = env::var("DATABASE_MIN_CONNECTIONS")
            .expect("DATABASE_MIN_CONNECTIONS not found at .env file")
            .parse::<u32>()
            .expect("DATABASE_MIN_CONNECTIONS must be a number");

        ConnectOptions::new(db_url)
            .max_connections(db_max_connections)
            .min_connections(db_min_connections)
            .to_owned()
    })
}
