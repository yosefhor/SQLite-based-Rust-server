use std::env;

#[derive(Clone)]
pub struct Config {
    pub database_url: String,
    pub server_port: u16,
}

impl Config {
    pub fn load() -> Self {
        if dotenvy::dotenv().is_err() {
            tracing::warn!(".env file not found, using environment/defaults");
        }

        let database_url = env::var("DATABASE_URL").unwrap_or_else(|_| {
            tracing::warn!("DATABASE_URL not set, using sqlite://items.db");
            "sqlite://items.db".to_string()
        });

        let server_port = env::var("SERVER_PORT")
            .ok()
            .and_then(|v| v.parse().ok())
            .unwrap_or_else(|| {
                tracing::warn!("SERVER_PORT invalid or missing, using 3000");
                3000
            });

        Self {
            database_url,
            server_port,
        }
    }
}
