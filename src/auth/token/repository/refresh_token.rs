use std::sync::Arc;

use fred::prelude::Client;

pub struct RedisRefreshTokenRepository {
    client: Arc<Client>,
}

impl RedisRefreshTokenRepository {
    pub fn new(client: Arc<Client>) -> RedisRefreshTokenRepository {
        return RedisRefreshTokenRepository { client };
    }
}
