mod commands;
mod config;
mod state;

use poise::serenity_prelude as serenity;
use serenity::prelude::GatewayIntents;

use crate::config::ConfigEnv;
use crate::state::{Data, Error};

#[tokio::main]
async fn main() {
    let config = ConfigEnv::load();
    let intents = GatewayIntents::all();

    let framework = poise::Framework::<Data, Error>::builder()
        .options(poise::FrameworkOptions {
            commands: commands::get_commands(),
            prefix_options: poise::PrefixFrameworkOptions {
                prefix: Some(config.prefix.clone()),
                ..Default::default()
            },
            ..Default::default()
        })
        .setup({
            let config = config.clone();

            move |_ctx, ready, _framework| {
                Box::pin(async move {
                    println!("{} está online!", ready.user.name);

                    Ok(Data {
                        config
                    })
                })
            }
        })
        .build();

    let mut client = serenity::Client::builder(config.token.clone(), intents)
        .framework(framework)
        .await
        .expect("Erro ao criar client");

    println!("Bot iniciando...");
    client.start().await.expect("Erro ao iniciar bot");
}