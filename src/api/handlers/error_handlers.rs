use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;

pub async fn handle_404() -> impl IntoResponse {
    let body = Json(serde_json::json!(
        { "success": false, "msg": "The requested content is not available." }
    ));
    (StatusCode::NOT_FOUND, body).into_response()
}
