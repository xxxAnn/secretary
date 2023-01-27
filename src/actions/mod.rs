pub mod message;
pub mod role;
pub mod user;

pub use message::*;
pub use role::*;
pub use user::*;

macro_rules! unwrap {
    ($value:expr, $pattern:pat => $result:expr) => {
        match $value {
            VoteAction::MessageCreate($pattern) => $result,
            VoteAction::MessageDelete($pattern) => $result,
            VoteAction::RoleCreate($pattern) => $result,
            VoteAction::RoleDelete($pattern) => $result,
            VoteAction::UserRoleAdd($pattern) => $result
        }
    };
}

#[derive(Debug)]
pub enum VoteAction {
    MessageCreate(MessageCreateAction),
    MessageDelete(MessageDeleteAction),
    RoleCreate(RoleCreateAction),
    RoleDelete(RoleDeleteAction),
    UserRoleAdd(UserRoleAddAction)
}

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
        unwrap!(self, ref mut m => m.finished = true)
    }
    pub fn set_ogmsg(&mut self, ogmsg: u64) {
        unwrap!(self, ref mut m => m.ogmsg = ogmsg)
    }
    pub fn already_voted(&mut self, r: u64, vote: bool) -> i16 {
        let already_voted;
        unwrap!(self, ref mut m => already_voted = &mut m.already_voted);
        let mut factor = 1;
        let k = already_voted.contains(&(r, vote));
        if k { return 0 }

        if let Some(index) = already_voted.iter().position(|x| x == &(r, !vote)) {
            already_voted.remove(index);
            factor = 2;
        }

        already_voted.push((r, vote));

        factor
    }
}