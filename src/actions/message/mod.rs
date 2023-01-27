use crate::*;

mod create;
mod delete;

pub use create::*;
pub use delete::*;

#[poise::command(slash_command, subcommands("create", "delete"))]
pub async fn message(_: Context<'_>) -> Result<(), Error> {
    Ok(())
}

