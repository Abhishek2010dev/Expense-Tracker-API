use crate::auth::token::repository::refresh_token::RefreshTokenRepository;

pub struct RefreshTokenService<R: RefreshTokenRepository> {
    repository: R,
}

impl<R: RefreshTokenRepository> RefreshTokenService<R> {
    pub fn new(repository: R) -> Self {
        return Self { repository };
    }
}
