mod commands;
mod config;

use poise::serenity_prelude as serenity;
use serenity::prelude::GatewayIntents;
use crate::config::ConfigEnv;

type Error = Box<dyn std::error::Error + Send + Sync>;

#[tokio::main]
async fn main() {

    let config: ConfigEnv = ConfigEnv::load();
    let intents: GatewayIntents = serenity::GatewayIntents::all();

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: commands::get_commands(),
            prefix_options: poise::PrefixFrameworkOptions {
                prefix: Some(config.prefix.into()),
                ..Default::default()
            },
            ..Default::default()
        })
        .setup(|_ctx, ready, _framework| {
            Box::pin(async move {
                println!("{} está online!", ready.user.name);
                Ok(())
            })
        })
        .build();

    let mut client = serenity::Client::builder(config.token, intents)
        .framework(framework)
        .await
        .unwrap();

    println!("Bot iniciando...");
    client.start().await.unwrap();
}