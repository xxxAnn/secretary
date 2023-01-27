use crate::*;

#[poise::command(slash_command, subcommands())]
pub async fn role(_: Context<'_>) -> Result<(), Error> {
    Ok(())
}

