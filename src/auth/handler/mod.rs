use std::sync::Arc;

use axum::{Router, routing::post};
use login::login_handler;
use refresh::refresh_token_handler;
use register::register_handler;

use crate::state::AppState;

pub mod login;
pub mod logout;
pub mod refresh;
pub mod register;

pub fn router() -> Router<Arc<AppState>> {
    return Router::new()
        .route("/register", post(register_handler))
        .route("/login", post(login_handler))
        .route("/refresh", post(refresh_token_handler));
}
