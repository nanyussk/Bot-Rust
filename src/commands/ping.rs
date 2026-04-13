use std::time::Instant;
use sysinfo::{Pid, ProcessesToUpdate, System};

type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, (), Error>;



///Sincronizar os comandos do bot.
#[poise::command(
    slash_command,
)]
pub async fn ping(ctx: Context<'_>) -> Result<(), Error> {

    let start = Instant::now();

    let mut system = System::new();
    let pid = Pid::from_u32(std::process::id());

    system.refresh_processes(ProcessesToUpdate::Some(&[pid]), true);

    let (memory, cpu) = if let Some(process) = system.process(pid) {
        (
            process.memory() as f64 / 1024.0 / 1024.0,
            process.cpu_usage()
        )
    } else {
        (0.0, 0.0)
    };

    let latency = start.elapsed().as_millis();

    ctx.say(format!(
        "🏓 **Pong!**\n\
        ⏱️ {}ms\n\
        💾 {:.2} MB\n\
        ⚙️ {:.2}%",
        latency, memory, cpu
    )).await?;

    Ok(())
}