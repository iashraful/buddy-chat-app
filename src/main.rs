use deadpool_diesel::{Manager, Pool};
use diesel::PgConnection;
use dotenvy::dotenv;

use api::server::run_server;
use core::config::config;
use core::db;
use core::logging::LogManager;

pub mod api;
pub mod chat;
pub mod core;

#[tokio::main]
async fn main() {
    dotenv().ok();
    // Initialize the config
    let config = config().await;
    // Enable Logger
    let logger = LogManager { config };
    logger.init_logging();

    // Connecting to Postgres DB
    let conn_pool: Pool<Manager<PgConnection>> =
        db::establish_connection(config.db_url().to_string());

    run_server(config, conn_pool).await;
}
