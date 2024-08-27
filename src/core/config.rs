use std::env;

use log::LevelFilter;
use tokio::sync::OnceCell;

#[derive(Debug)]
struct ServerConfig {
    host: String,
    port: u16,
    log_level: LevelFilter,
}

#[derive(Debug)]
struct DBConfig {
    url: String,
}

#[derive(Debug)]
pub struct Config {
    server: ServerConfig,
    db: DBConfig,
}

impl Config {
    pub fn db_url(&self) -> &str {
        &self.db.url
    }

    pub fn server_host(&self) -> &str {
        &self.server.host
    }

    pub fn server_port(&self) -> u16 {
        self.server.port
    }

    pub fn log_level(&self) -> LevelFilter {
        self.server.log_level
    }
}

pub static CONFIG: OnceCell<Config> = OnceCell::const_new();

async fn init_config() -> Config {
    let server_config = ServerConfig {
        host: env::var("API_HOST").unwrap_or_else(|_| String::from("127.0.0.1")),
        port: env::var("API_PORT")
            .unwrap_or_else(|_| String::from("3000"))
            .parse::<u16>()
            .unwrap(),
        log_level: env::var("LOG_LEVEL").unwrap().parse::<LevelFilter>().unwrap_or(LevelFilter::Debug),
    };

    let database_config = DBConfig {
        url: env::var("DATABASE_URL").expect("DATABASE_URL must be set"),
    };

    Config {
        server: server_config,
        db: database_config,
    }
}

pub async fn config() -> &'static Config {
    CONFIG.get_or_init(init_config).await
}