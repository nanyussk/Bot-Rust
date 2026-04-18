pub mod ping;
pub mod sync;

use crate::state::{Data, Error};

pub fn get_commands() -> Vec<poise::Command<Data, Error>> {
    vec![
        ping::ping(),
        sync::sync(),
    ]
}