mod core;
mod commands;

#[tokio::main]
async fn main() {
    core::run().await;
}