pub mod role;

pub use role::*;

use crate::{Context, Error};

#[poise::command(slash_command, subcommands("role_add"))]
pub async fn user(_: Context<'_>) -> Result<(), Error> {
    Ok(())
}

