pub struct JwtService {
    token: String,
}

impl JwtService {
    pub fn new(token: impl Into<String>) -> Self {
        return Self {
            token: token.into(),
        };
    }
}
