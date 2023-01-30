/*
    Copyright (C) 2023 Ann Mauduy-Decius

    This program is free software: you can redistribute it and/or modify
    it under the terms of the GNU General Public License as published by
    the Free Software Foundation, either version 3 of the License, or
    (at your option) any later version.

    This program is distributed in the hope that it will be useful,
    but WITHOUT ANY WARRANTY; without even the implied warranty of
    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
    GNU General Public License for more details.

    You should have received a copy of the GNU General Public License
    along with this program.  If not, see <https://www.gnu.org/licenses/>.
*/

pub mod can_view_send;
pub mod cant_send;
pub mod cant_view;
pub mod create;
pub mod delete;
pub mod set_position;
pub mod hoist;

pub use can_view_send::*;
pub use cant_send::*;
pub use cant_view::*;
pub use create::*;
pub use delete::*;
pub use hoist::*;
pub use set_position::*;

use crate::{Context, Error};

#[poise::command(
    slash_command,
    subcommands(
        "role_create",
        "role_delete",
        "can_view_send",
        "cant_send",
        "cant_view",
        "role_set_position",
        "role_hoist",
        "role_unhoist"
    )
)]
pub async fn role(_: Context<'_>) -> Result<(), Error> {
    Ok(())
}
