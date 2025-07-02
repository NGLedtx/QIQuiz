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
        let expiration = usize::MAX;

        JwtOpts { secret, expiration }
    })
}
