pub mod ping;
pub mod sync;

use poise::Command;

use crate::{Error};

pub fn get_commands() -> Vec<Command<(), Error>> {
    vec![
        ping::ping(),
        sync::sync(),
    ]
}