use std::{env, sync::OnceLock};

struct AppConfig {
    port: u16,
}

static APP_CONFIG: OnceLock<AppConfig> = OnceLock::new();

pub fn get_app_config() -> &'static AppConfig {
    APP_CONFIG.get_or_init(|| {
        let port = env::var("APP_PORT")
            .expect("APP_PORT not found at .env file")
            .parse()
            .expect("APP_PORT needs be a number");

        AppConfig { port }
    })
}
