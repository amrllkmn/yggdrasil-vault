use crate::handler;
use axum::{routing::get, Router};
pub fn create_api_route() -> Router {
    let router = Router::new().route("/decrypt", get(handler::decrypt));
    router
}
