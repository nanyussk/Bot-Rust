pub mod general;
pub mod developer;

use crate::core::{Data, Error};

pub fn get_commands() -> Vec<poise::Command<Data, Error>> {
    vec![
        general::ping::ping(),
        general::botinfo::botinfo(),
        developer::sync::sync(),
    ]
}