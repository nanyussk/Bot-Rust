use poise::serenity_prelude as serenity;
use serenity::prelude::GatewayIntents;

use tracing::info;

use super::config::ConfigEnv;

use super::framework;

pub async fn build() -> serenity::Client {
    let config = ConfigEnv::load();

    info!("Carregando configuração...");

    let intents = GatewayIntents::all();

    info!("Criando framework...");

    let framework = framework::build(config.clone());

    info!("Criando client...");

    let client = serenity::Client::builder(
        config.token.clone(),
        intents
    )
    .framework(framework)
    .await
    .expect("Erro ao criar client");

    info!("Client criado com sucesso");

    client
}