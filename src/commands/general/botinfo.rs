use poise::serenity_prelude::{Colour, CreateEmbed, CreateEmbedFooter};
use sysinfo::System;

use crate::core::{Context, Error};

///Mostra informações sobre o bot.
#[poise::command(slash_command)]
pub async fn botinfo(ctx: Context<'_>) -> Result<(), Error> {
    let bot_cache = ctx.serenity_context().cache.clone();
    let developer_owner = ctx.serenity_context().http.get_user(804509819254866000.into()).await?;

    let embed = CreateEmbed::new()
    .title(format!("Informações sobre {}", bot_cache.current_user().name.clone()))
    .color(Colour::new(4927093))
    .description(format!(
        "```yaml\n\
        SO:         {:?}\n\
        Rust:       {}\n\
        Serenity:   {}\n\
        Poise:      {}\n\
        ```\n\
        ### 📊 Estatísticas (Cache)
        **Servidores:** `{}`\n\
        **Usuários:** `{}`\n\
        ",
        System::long_os_version().unwrap(),
        env!("CARGO_PKG_VERSION"),
        "next (git)",
        "0.6.1",
        bot_cache.guild_count(),
        bot_cache.user_count()
    ))
    .thumbnail(bot_cache.current_user().avatar_url().unwrap_or_default())
    .footer(
        CreateEmbedFooter::new(format!("Desenvolvida por {} ({})", developer_owner.name, developer_owner.id))
        .icon_url(&developer_owner.avatar_url().unwrap_or_default())
    );

    ctx.send(
        poise::CreateReply::default()
        .embed(embed)
        .ephemeral(true)
    ).await?;

    Ok(())
}

