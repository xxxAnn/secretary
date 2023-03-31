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

use log::LevelFilter;

pub const INFO_CHANNEL: u64 = 970418738023235584;
pub const GUILD_ID: u64 = 969_011_076_144_431_165;
pub const VOTE_CHANNEL: u64 = 970_108_683_746_951_178;
pub const LOG_CHANNEL: u64 = 970419838604439642;
pub const DEFAULT_NRQ: i16 = 2;
pub const SESSION_NUMBER: u16 = 58;
pub const LEVEL: LevelFilter = LevelFilter::Info;
pub const VOTER_ROLE: u64 = 1069130087116578908;
pub const PROPOSE_CHANNEL: u64 = 1090509087751536702;
// These roles cannot be deleted and their permissions cannot be modified.
pub const CANNOT_MODIFY: &'static [u64] = &[1069130087116578908, 1069432736982499438];
// These roles
pub const CANNOT_MODIFY_CHANNEL: &'static [u64] = &[1090509087751536702, 970108683746951178];
// This is the moderator role, it only has permissions when the bot isn't in session.
pub const GENERAL_SECRETARY_ROLE: u64 = 1069432736982499438;
