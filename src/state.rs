use crate::config::ConfigEnv;

pub struct Data {
    #[allow(dead_code)]
    pub config: ConfigEnv
}

pub type Error = Box<dyn std::error::Error + Send + Sync>;
pub type Context<'a> = poise::Context<'a, Data, Error>;