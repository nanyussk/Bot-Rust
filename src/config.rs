use std::env;
use dotenv::dotenv;

pub struct ConfigEnv {
    pub token: String,
    pub prefix: String
}

impl ConfigEnv {
    pub fn load() -> Self {
        dotenv().ok();

        Self {
            token: env::var("TOKEN").expect("Token do bot não encontrado"),
            prefix: env::var("PREFIX").expect("Prefixo do bot não encontrado")
        }
    }
}