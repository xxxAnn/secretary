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
pub struct RoleDelete {
    role_id: u64,
    votes: i16,
    pub ogmsg: u64,
    pub already_voted: Vec<(u64, bool)>,
    pub finished: bool,
}

impl RoleDelete {
    pub fn handle(&mut self, p: i16) -> i16 {
        self.votes += p;

        self.votes
    }
    pub async fn call(self, http: impl AsRef<Http>) {
        if let Err(e) = serenity::GuildId(consts::GUILD_ID)
            .delete_role(&http, self.role_id)
            .await
        {
            error!("Failed to delete role. {:?}", e)
        } else if let Err(e) = serenity::ChannelId(consts::VOTE_CHANNEL)
            .send_message(&http, |msg| {
                msg.content("Vote passed.").reference_message((
                    serenity::ChannelId(consts::VOTE_CHANNEL),
                    serenity::MessageId(self.ogmsg),
                ))
            })
            .await
        {
            error!("Failed to announce vote success. {:?}", e)
        }
    }
    pub fn action(self) -> VoteAction {
        VoteAction::RoleDelete(self)
    }
}

#[poise::command(slash_command, rename = "delete")]
pub async fn role_delete(
    ctx: Context<'_>,
    #[description = "Role to delete"] role: serenity::Role,
) -> Result<(), Error> {
    info!(
        "Received command by user named {}#{} with user id {}.",
        ctx.author().name,
        ctx.author().discriminator,
        ctx.author().id.0
    );
    debug!("Received context object {:?}.", &ctx);
    if vec![1_069_130_087_116_578_908].contains(&role.id.0) {
        let _ = ctx
            .send(|m| m.content("You cannot delete this role.").ephemeral(true))
            .await;
        Ok(())
    } else {
        create_vote(
            &ctx,
            format!("Delete role <@&{}>", &role.id.0),
            VoteAction::RoleDelete(RoleDelete {
                role_id: role.id.0,
                votes: 0,
                ogmsg: 0,
                already_voted: vec![],
                finished: false,
            }),
        )
        .await
    }
}
