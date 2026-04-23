pub mod sync;

use crate::core::{Data, Error};

pub fn commands() -> Vec<poise::Command<Data, Error>> {
    vec![
        sync::sync(),
    ]
}