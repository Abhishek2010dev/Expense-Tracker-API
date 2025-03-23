use std::sync::Arc;

use axum::{
    Router,
    routing::{delete, post},
};
use login::login_handler;
use logout::logout_handler;
use refresh::refresh_token_handler;
use register::register_handler;

use crate::state::AppState;

pub mod login;
pub mod logout;
pub mod refresh;
pub mod register;

pub fn router() -> Router<Arc<AppState>> {
    return Router::new()
        .route("/auth/register", post(register_handler))
        .route("/auth/login", post(login_handler))
        .route("/auth/refresh", post(refresh_token_handler))
        .route("/auth/logout", delete(logout_handler));
}
