use std::env;

#[derive(Clone)]
pub struct Config {
    pub database_url: String,
    pub server_port: u16,
}

impl Config {
    pub fn load() -> Self {
        dotenvy::dotenv().ok();

        let database_url =
            env::var("DATABASE_URL").unwrap_or("sqlite://items.db".to_string());

        let server_port =
            env::var("SERVER_PORT").ok()
                .and_then(|v| v.parse().ok())
                .unwrap_or(3000);

        Self {
            database_url,
            server_port,
        }
    }
}
