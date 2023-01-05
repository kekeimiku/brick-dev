use once_cell::sync::Lazy;
use std::env;

pub struct Config {
    pub jwt_secret: String,
    pub jwt_exp: u64,
    pub jwt_sub: String,
    pub jwt_company: String,
    pub pg_conn: String,
    pub socket_addr: String,
    pub redis_conn: String,
}

pub static CONFIG: Lazy<Config> = Lazy::new(|| {
    let jwt_secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set");
    let jwt_exp = env::var("JWT_EXP")
        .expect("JWT_EXP must be set")
        .parse()
        .expect("invalid string");
    let jwt_sub = env::var("JWT_SUB").expect("JWT_SUB must be set");
    let jwt_company = env::var("JWT_COMPANY").expect("JWT_COMPANY must be set");
    let pg_conn = env::var("PG_CONN").expect("PG_CONN must be set");
    let socket_addr = env::var("SOCKET_ADDR").expect("SOCKET_ADDR must be set");
    let redis_conn = env::var("REDIS_CONN").expect("REDIS_CONN must be set");
    Config {
        jwt_secret,
        jwt_exp,
        jwt_sub,
        jwt_company,
        pg_conn,
        socket_addr,
        redis_conn,
    }
});
