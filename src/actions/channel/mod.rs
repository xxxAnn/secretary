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
