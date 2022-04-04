use anyhow::Result;

pub struct Credentials {
    pub username: String,
    pub password: String,
    pub server: String,
}

pub struct Client {}

impl Client {
    pub async fn login(credentials: Credentials) -> anyhow::Result<Self> {
        Ok(Self {})
    }
}
