use super::config::ConfigEnv;

pub struct Data {
    #[allow(dead_code)]
    pub config: ConfigEnv,
    #[allow(dead_code)]
    pub http: reqwest::Client
}

pub type Error = Box<dyn std::error::Error + Send + Sync>;
pub type Context<'a> = poise::Context<'a, Data, Error>;