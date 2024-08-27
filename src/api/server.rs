use crate::{api::routes::create_api_router, core::config::Config};
use deadpool_diesel::{Manager, Pool};
use diesel::PgConnection;
use tokio::net::TcpListener;

pub async fn run_server(config: &Config, pool: Pool<Manager<PgConnection>>) {
    let host: &str = config.server_host();
    let port: u16 = config.server_port();
    let address: String = format!("{}:{}", host, port);

    let router: axum::Router = create_api_router(pool);
    println!("Listening on http://{}", address);
    let listener: TcpListener = TcpListener::bind(address).await.unwrap();
    axum::serve(listener, router).await.unwrap();
}
