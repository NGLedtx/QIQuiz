use std::{env, sync::OnceLock};

pub struct AppConfig {
    pub port: u16,
    pub user_admin: String,
    pub user_password: String,
}

static APP_CONFIG: OnceLock<AppConfig> = OnceLock::new();

pub fn get_app_config() -> &'static AppConfig {
    APP_CONFIG.get_or_init(|| {
        let port = env::var("APP_PORT")
            .expect("APP_PORT not found at .env file")
            .parse()
            .expect("APP_PORT needs be a number");
        let user_admin = env::var("USER_ADMIN").expect("USER_ADMIN not found at .env file");
        let user_password =
            env::var("USER_PASSWORD").expect("USER_PASSWORD not found at .env file");

        AppConfig {
            port,
            user_admin,
            user_password,
        }
    })
}
