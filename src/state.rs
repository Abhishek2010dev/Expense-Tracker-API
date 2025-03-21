use std::sync::Arc;

use sqlx::PgPool;

use crate::{
    auth::token::{
        repository::refresh_token::RedisRefreshTokenRepository,
        service::{access_token::AccessTokenService, refresh_token::RefreshTokenService},
    },
    config::Config,
    user::repository::{UserRespository, UserRespositoryImpl},
};
use fred::prelude::Client as RedisClient;

pub struct AppState {
    user_repository: Arc<dyn UserRespository>,
    access_token_service: Arc<dyn TokenService>,
    refresh_token_service: Arc<dyn TokenService>,
}

impl AppState {
    pub fn new<C: Config>(db: Arc<PgPool>, redis_client: Arc<RedisClient>, config: C) -> AppState {
        let user_repo = Arc::new(UserRepositoryImpl::new(db)) as Arc<dyn UserRepository>;
        let refresh_token_repo = RedisRefreshTokenRepository::new(redis_client);
        let access_token_service =
            Arc::new(AccessTokenService::new(config.access_secret())) as Arc<dyn TokenService>;
        let refresh_token_service = Arc::new(RefreshTokenService::new(
            refresh_token_repo,
            config.refresh_secret(),
        )) as Arc<dyn TokenService>;

        AppState {
            user_repository: user_repo,
            access_token_service,
            refresh_token_service,
        }
    }
}
