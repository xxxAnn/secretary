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

pub mod role;
pub mod remove;

pub use remove::*;
pub use role::*;


use crate::{Context, Error};

#[poise::command(slash_command, subcommands("role_add", "role_remove", "user_kick"))]
pub async fn user(_: Context<'_>) -> Result<(), Error> {
    Ok(())
}
