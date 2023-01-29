mod textcreate;
mod voicecreate;
mod delete;
mod categorycreate;

pub use textcreate::*;
pub use delete::*;
pub use voicecreate::*;
pub use categorycreate::*;

use crate::{Context, Error};

#[poise::command(slash_command, subcommands("create_text", "create_voice", "channel_delete", "create_category"))]
pub async fn channel(_: Context<'_>) -> Result<(), Error> {
    Ok(())
}

