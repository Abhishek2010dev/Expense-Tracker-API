use super::{model::User, utils::CreateUserPayload};
use anyhow::Result;

pub trait UserRespository {
    async fn create(&self, payload: CreateUserPayload) -> Result<User>;
    async fn find_by_id(&self, id: i32) -> Result<Option<User>>;
    async fn exits(&self, id: i32) -> Result<bool>;
}
