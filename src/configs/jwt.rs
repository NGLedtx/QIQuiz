use std::{env, sync::OnceLock};

#[derive(Clone)]
pub struct JwtOpts {
    pub secret: String,
    pub expiration: usize,
}

static JWT_OPTS: OnceLock<JwtOpts> = OnceLock::new();

pub fn get_jwt_opts() -> &'static JwtOpts {
    JWT_OPTS.get_or_init(|| {
        let secret = env::var("JWT_SECRET").expect("JWT_SECRET not found at .env file");
        let expiration = env::var("JWT_EXPIRATION")
            .expect("JWT_EXPIRATION not found at .env file")
            .parse()
            .expect("JWT_EXPIRATION needs be a number");

        JwtOpts { secret, expiration }
    })
}
