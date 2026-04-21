use std::time::Instant;
use poise::{CreateReply};
use sysinfo::{Pid, ProcessesToUpdate, System};

use crate::core::{Context, Error};

/// Mostra informações sobre a latência do bot.
#[poise::command(slash_command)]
pub async fn ping(ctx: Context<'_>) -> Result<(), Error> {
    let start = Instant::now();

    let message = ctx.say("🏓 | Ping...").await?;

    let latency = start.elapsed().as_millis();

    let mut system = System::new();
    let pid = Pid::from_u32(std::process::id());

    system.refresh_processes(ProcessesToUpdate::Some(&[pid]), true);

    let (memory, cpu) = if let Some(process) = system.process(pid) {
        (
            process.memory() as f64 / 1024.0 / 1024.0, // MB
            process.cpu_usage(),
        )
    } else {
        (0.0, 0.0)
    };

    message
        .edit(
            ctx,
            CreateReply::default().content(format!(
                "🏓 | Pong!\n\n\
                ⏱️ Latência: `{latency}ms`\n\
                💾 Memória: `{:.2} MB`\n\
                ⚙️ CPU: `{:.2}%`",
                memory, cpu
            )),
        )
        .await?;

    Ok(())
}