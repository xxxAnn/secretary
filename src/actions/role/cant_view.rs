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
pub struct CantView {
    channel_id: u64,
    role_id: u64,
    votes: i16,
    pub ogmsg: u64,
    pub already_voted: Vec<(u64, bool)>,
    pub finished: bool,
}

impl CantView {
    pub fn handle(&mut self, p: i16) -> i16 {
        self.votes += p;

        self.votes
    }
    pub async fn call(self, http: impl AsRef<Http>) {
        if let Err(e) = serenity::ChannelId(self.channel_id)
            .create_permission(
                &http,
                &serenity::PermissionOverwrite {
                    allow: serenity::Permissions::empty(),
                    deny: serenity::Permissions::VIEW_CHANNEL,
                    kind: serenity::PermissionOverwriteType::Role(serenity::RoleId(self.role_id)),
                },
            )
            .await
        {
            error!("Failed to edit role permissions. {:?}", e)
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
        VoteAction::CantView(self)
    }
}

#[poise::command(
    slash_command,
    description_localized("en-US", "Prevents role from viewing the selected channel.")
)]
pub async fn cant_view(
    ctx: Context<'_>,
    #[description = "Role to block from the channel"] role: serenity::Role,
    #[description = "Channel to block the role from seeing"] channel: serenity::GuildChannel,
) -> Result<(), Error> {
    info!(
        "Received command by user named {}#{} with user id {}.",
        ctx.author().name,
        ctx.author().discriminator,
        ctx.author().id.0
    );
    debug!("Received context object {:?}.", &ctx);

    if Vec::from(consts::CANNOT_MODIFY).contains(&role.id.0) {
        let _ = ctx
            .send(|m| m.content("You cannot modify this role.").ephemeral(true))
            .await;
        Ok(())
    } else if Vec::from(consts::CANNOT_MODIFY_CHANNEL).contains(&channel.id.0) {
        let _ = ctx
            .send(|m| {
                m.content("You cannot delete these channels.")
                    .ephemeral(true)
            })
            .await;
        Ok(())
    } else {
        create_vote(
            &ctx,
            format!(
                "Block role <@&{}> from seeing <#{}>",
                &role.id.0, &channel.id.0
            ),
            VoteAction::CantView(CantView {
                channel_id: channel.id.0,
                role_id: role.id.0,
                ogmsg: 0,
                votes: 1,
                already_voted: vec![],
                finished: false,
            }),
        )
        .await
    }
}
