pub mod client;
pub mod config;
pub mod framework;
pub mod logging;
pub mod state;

pub use state::*;

use tracing::{info, error};

pub async fn run() {
    logging::init();

    info!("Iniciando bot...");

    let mut client = client::build().await;

    match client.start().await {
        Ok(_) => {}
        Err(err) => {
            error!(error = ?err, "Erro ao iniciar bot");
        }
    }
}