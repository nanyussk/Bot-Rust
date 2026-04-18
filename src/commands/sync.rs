use crate::state::{Context, Error};

#[poise::command(prefix_command, owners_only)]
pub async fn sync(ctx: Context<'_>) -> Result<(), Error> {
    poise::builtins::register_application_commands(ctx, true).await?;

    Ok(())
}