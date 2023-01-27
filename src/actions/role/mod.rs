pub mod create;
pub mod delete;

pub use create::*;
pub use delete::*;

use crate::{Context, Error};

#[poise::command(slash_command, subcommands("role_create", "role_delete"))]
pub async fn role(_: Context<'_>) -> Result<(), Error> {
    Ok(())
}

