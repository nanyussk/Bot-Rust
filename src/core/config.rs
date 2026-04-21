use dotenv::dotenv;

#[derive(Clone)]
pub struct ConfigEnv {
    pub token: String,
    pub prefix: String,
}

impl ConfigEnv {
    pub fn load() -> Self {
        dotenv().ok();

        Self {
            token: std::env::var("DISCORD_TOKEN")
                .expect("DISCORD_TOKEN não encontrado"),
            prefix: std::env::var("PREFIX")
                .unwrap_or_else(|_| "!".into()),
        }
    }
}