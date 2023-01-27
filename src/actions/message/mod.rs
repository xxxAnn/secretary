use crate::*;

mod create;

pub use create::*;

#[poise::command(slash_command, subcommands("create"))]
pub async fn message(_: Context<'_>) -> Result<(), Error> {
    Ok(())
}

