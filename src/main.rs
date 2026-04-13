use poise::serenity_prelude as serenity;
use dotenv::dotenv;
use std::env;

mod commands;

type Error = Box<dyn std::error::Error + Send + Sync>;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let token = env::var("BOT_TOKEN").expect("Token não encontrado");
    let prefix = env::var("PREFIX").expect("Prefixo não encontrado");

    let intents = serenity::GatewayIntents::all();

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: commands::get_commands(),
            prefix_options: poise::PrefixFrameworkOptions {
                prefix: Some(prefix.into()),
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

    let mut client = serenity::Client::builder(token, intents)
        .framework(framework)
        .await
        .unwrap();

    println!("Bot iniciando...");
    client.start().await.unwrap();
}