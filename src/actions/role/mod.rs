pub mod create;
pub mod delete;
pub mod cant_send;
pub mod cant_view;
pub mod can_view_send;

pub use create::*;
pub use delete::*;
pub use cant_send::*;
pub use cant_view::*;
pub use can_view_send::*;

use crate::{Context, Error};

#[poise::command(slash_command, subcommands("role_create", "role_delete", "can_view_send", "cant_send", "cant_view"))]
pub async fn role(_: Context<'_>) -> Result<(), Error> {
    Ok(())
}

