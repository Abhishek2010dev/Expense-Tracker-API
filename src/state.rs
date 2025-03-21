use std::sync::Arc;

use sqlx::PgPool;

use crate::{
    auth::token::{
        repository::refresh_token::RedisRefreshTokenRepository,
        service::{access_token::AccessTokenService, refresh_token::RefreshTokenService},
    },
    config::{Config, env_config::EnvConfig},
    user::repository::UserRespositoryImpl,
};
use fred::prelude::Client as RedisClient;

pub struct AppState {
    user_repository: UserRespositoryImpl,
    access_token_service: AccessTokenService,
    refresh_token_service: RefreshTokenService<RedisRefreshTokenRepository>,
}

impl AppState {
    pub fn new(db: Arc<PgPool>, redis_client: Arc<RedisClient>, config: EnvConfig) -> AppState {
        let refresh_token_repo = RedisRefreshTokenRepository::new(redis_client);
        return AppState {
            user_repository: UserRespositoryImpl::new(db),
            access_token_service: AccessTokenService::new(config.access_secret()),
            refresh_token_service: RefreshTokenService::new(config., secret_key),
        };
    }
}
