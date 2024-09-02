use axum::http::header::{ACCEPT, AUTHORIZATION, ORIGIN};
use axum::http::Method;
use axum::{routing::get, Router};
use deadpool_diesel::{Manager, Pool};
use diesel::PgConnection;
use log::info;
use socketioxide::SocketIo;
use tower_http::cors::{Any, CorsLayer};

use crate::api::handlers::common_handlers;
use crate::api::handlers::error_handlers;
use crate::chat::services::soc_on_connect;

pub fn create_api_router(pool: Pool<Manager<PgConnection>>) -> Router {
    info!("Creating socket layer");
    let (soc_layer, io) = SocketIo::new_layer();
    io.ns("/", soc_on_connect);
    
    let cors_layer = CorsLayer::new()
        .allow_methods(vec![Method::GET, Method::POST, Method::PUT, Method::DELETE])
        .allow_headers(vec![ORIGIN, AUTHORIZATION, ACCEPT])
        .allow_origin(Any);

    Router::new()
        .route("/", get(common_handlers::index))
        .layer(cors_layer)
        .layer(soc_layer)
        .fallback(error_handlers::handle_404)
        .with_state(pool)
}
