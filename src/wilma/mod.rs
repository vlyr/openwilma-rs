use serde::{Deserialize, Serialize};

pub mod user;
pub use user::User;

pub mod schedule;
pub use schedule::Schedule;

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all(deserialize = "PascalCase"))]
pub struct IndexResponse {
    login_result: String,
    #[serde(rename(deserialize = "SessionID"))]
    session_id: String,
    api_version: u32,
}

impl IndexResponse {
    pub fn api_version(&self) -> u32 {
        self.api_version
    }

    pub fn login_result(&self) -> &String {
        &self.login_result
    }

    pub fn session_id(&self) -> &String {
        &self.session_id
    }
}
