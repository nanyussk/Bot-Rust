pub mod general;
pub mod developer;

use crate::core::{Data, Error};

pub fn build_commands() -> Vec<poise::Command<Data, Error>> {
    let mut commands = Vec::new();

    commands.extend(general::commands());
    commands.extend(developer::commands());

    commands
}