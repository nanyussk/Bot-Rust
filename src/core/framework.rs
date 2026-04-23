use tracing::info;

use crate::commands;
use super::config::ConfigEnv;
use super::state::{Data, Error};

pub fn build(config: ConfigEnv) -> poise::Framework<Data, Error> {
    poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: commands::build_commands(),

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
                    info!(
                        "Conetada(a) como {}",
                        ready.user.name
                    );

                    Ok(Data {
                        config,
                        http: reqwest::Client::new(),
                    })
                })
            }
        })
        .build()
}