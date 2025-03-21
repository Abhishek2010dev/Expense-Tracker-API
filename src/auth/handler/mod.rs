use std::sync::Arc;

use axum::{Router, routing::post};
use register::register_handler;

use crate::state::AppState;

pub mod login;
pub mod register;

pub fn router() -> Router<Arc<AppState>> {
    return Router::new().route("/register", post(register_handler));
}
