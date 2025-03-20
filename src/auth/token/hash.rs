use sha2::{Digest, Sha256};

pub fn hash_token(token: &str) -> String {
    let mut hasher = Sha256::default();
    hasher.update(token);
    format!("{:x}", hasher.finalize())
}
