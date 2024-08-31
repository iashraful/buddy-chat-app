use axum::{routing::get, Router};
use deadpool_diesel::{Manager, Pool};
use diesel::PgConnection;

use crate::api::handlers::common_handlers;
use crate::api::handlers::error_handlers;

pub fn create_api_router(pool: Pool<Manager<PgConnection>>) -> Router {
    Router::new()
        .route("/", get(common_handlers::index))
        .fallback(error_handlers::handle_404)
        .with_state(pool)
}
