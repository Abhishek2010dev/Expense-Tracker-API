use sqlx::PgPool;

use crate::{
    auth::token::{
        repository::refresh_token::RedisRefreshTokenRepository,
        service::{access_token::AccessTokenService, refresh_token::RefreshTokenService},
    },
    user::repository::UserRespositoryImpl,
};

pub struct AppState {
    user_repository: UserRespositoryImpl,
    access_token_service: AccessTokenService,
    refresh_token_service: RefreshTokenService<RedisRefreshTokenRepository>,
}

impl AppState {
    pub fn new(db: PgPool, redis_client: fred::prelude::Client) -> AppState {
        return AppState {
            user_repository: UserRespositoryImpl::new(db),
            access_token_service: (),
            refresh_token_service: (),
        };
    }
}
