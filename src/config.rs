use std::env;

pub struct Config {
    pub database_url: String,
    pub server_host: String,
    pub server_port: u16,
}

impl Config {
    pub fn load() -> Self {
        dotenvy::dotenv().ok();

        Self {
            database_url: env::var("DATABASE_URL").expect("DATABASE_URL must be set"),
            server_host: env::var("SERVER_HOST").unwrap_or_else(|_| "localhost".into()),
            server_port: env::var("SERVER_PORT")
                .unwrap_or("3000".into())
                .parse()
                .unwrap(),
        }
    }

    pub fn get_database_url(&self) -> &str {
        &self.database_url
    }

    pub fn get_server_host(&self) -> &str {
        &self.server_host
    }

    pub fn get_server_port(&self) -> u16 {
        self.server_port
    }
}
