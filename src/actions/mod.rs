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

pub mod channel;
pub mod message;
pub mod role;
pub mod user;

pub use channel::*;
pub use message::*;
pub use role::*;
pub use user::*;

macro_rules! generate_vote_actions {
    ($($x:ident),+) => {
        #[derive(Debug)]
        pub enum VoteAction {
            $($x($x)),+
        }

        macro_rules! unwrap {
            ($value:expr, $pattern:pat => $result:expr) => {
                match $value {
                    $(VoteAction::$x($pattern) => $result,)+
                }
            };
        }
    }
}

generate_vote_actions!(
    MessageCreate,
    MessageDelete,
    RoleCreate,
    RoleDelete,
    UserRoleAdd,
    CantSend,
    CantView,
    CanViewSend,
    TextChannelCreate,
    VoiceChannelCreate,
    ChannelDelete,
    UserRoleRemove,
    CategoryChannelCreate,
    ChannelPurge
);

/*
#[derive(Debug)]
pub enum VoteAction {
    MessageCreate(MessageCreateAction),
    MessageDelete(MessageDeleteAction),
    RoleCreate(RoleCreateAction),
    RoleDelete(RoleDeleteAction),
    UserRoleAdd(UserRoleAddAction),
}
*/

impl VoteAction {
    pub fn handle_tally(&mut self, p: i16) -> i16 {
        unwrap!(self, ref mut m => m.handle(p))
    }
    pub fn dummy(&self) -> Self {
        unwrap!(self, m => m.clone().action())
    }
    pub async fn call(self, http: impl AsRef<poise::serenity_prelude::Http>) {
        unwrap!(self, m => m.call(http).await);
    }
    pub fn is_finished(&self) -> bool {
        unwrap!(self, m => m.finished)
    }
    pub fn set_finished(&mut self) {
        unwrap!(self, ref mut m => m.finished = true);
    }
    pub fn set_ogmsg(&mut self, ogmsg: u64) {
        unwrap!(self, ref mut m => m.ogmsg = ogmsg);
    }
    pub fn already_voted(&mut self, r: u64, vote: bool) -> i16 {
        let already_voted = unwrap!(self, ref mut m => &mut m.already_voted);
        let mut factor = 1;
        let k = already_voted.contains(&(r, vote));
        if k {
            return 0;
        }

        if let Some(index) = already_voted.iter().position(|x| x == &(r, !vote)) {
            already_voted.remove(index);
            factor = 2;
        }

        already_voted.push((r, vote));

        factor
    }
}
