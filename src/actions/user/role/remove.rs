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

use crate::{consts, create_vote, debug, error, info, serenity, Context, Error, Http, VoteAction};

#[derive(Debug, Clone)]
pub struct UserRoleRemove {
    member_id: u64,
    role_id: u64,
    //
    votes: i16,
    pub ogmsg: u64,
    pub already_voted: Vec<(u64, bool)>,
    pub finished: bool,
}

impl UserRoleRemove {
    pub fn handle(&mut self, p: i16) -> i16 {
        self.votes += p;

        self.votes
    }
    pub async fn call(self, http: impl AsRef<Http>) {
        let mut user = http
            .as_ref()
            .get_member(consts::GUILD_ID, self.member_id)
            .await
            .unwrap();

        if let Err(e) = user.remove_role(&http, self.role_id).await {
            error!("Failed to remove role from user. {:?}", e);
        } else if let Err(e) = serenity::ChannelId(consts::VOTE_CHANNEL)
            .send_message(&http, |msg| {
                msg.content("Vote passed.").reference_message((
                    serenity::ChannelId(consts::VOTE_CHANNEL),
                    serenity::MessageId(self.ogmsg),
                ))
            })
            .await
        {
            error!("Failed to announce vote success. {:?}", e);
        }
    }
    pub fn action(self) -> VoteAction {
        VoteAction::UserRoleRemove(self)
    }
}

#[poise::command(slash_command)]
pub async fn role_remove(
    ctx: Context<'_>,
    #[description = "The user to remove the role from"] member: serenity::Member,
    #[description = "The role to remove from the user"] role: serenity::Role,
) -> Result<(), Error> {
    info!(
        "Received command by user named {}#{} with user id {}.",
        ctx.author().name,
        ctx.author().discriminator,
        ctx.author().id.0
    );
    debug!("Received a member object {:?}.", &member);
    debug!("Received context object {:?}.", &ctx);
    create_vote(
        &ctx,
        format!(
            "Remove role <@&{}> from user <@{}>",
            &role.id.0, &member.user.id.0
        ),
        VoteAction::UserRoleRemove(UserRoleRemove {
            member_id: member.user.id.0,
            role_id: role.id.0,
            votes: 0,
            ogmsg: 0,
            already_voted: vec![],
            finished: false,
        }),
    )
    .await
}
