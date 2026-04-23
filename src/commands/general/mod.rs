use crate::core::{Data, Error};

pub mod ping;
pub mod botinfo;

pub fn commands() -> Vec<poise::Command<Data, Error>> {
    vec![
        ping::ping(),
        botinfo::botinfo()
    ]
}