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

use crate::{consts, create_vote, vote_action, debug, error, serenity, Context, Error, Http, VoteAction};

vote_action!(
    RoleRename,
    move |me: RoleRename, http: std::sync::Arc<Http>| {
        async move {
            if let Err(e) = serenity::GuildId(me.role_id)
            .edit_role(&http, me.role_id, |e| {
                e.name(me.new_name)
            }).await {
                error!("Failed to rename role. {:?}", e)
            } else if let Err(e) = serenity::ChannelId(consts::VOTE_CHANNEL)
                .send_message(&http, |msg| {
                    msg.content("Vote passed.").reference_message((
                        serenity::ChannelId(consts::VOTE_CHANNEL),
                        serenity::MessageId(me.ogmsg),
                    ))
                })
                .await
            {
                error!("Failed to announce vote success. {:?}", e)
            }
        }
    },
    role_id: u64,
    new_name: String
);

#[poise::command(slash_command, rename = "rename")]
pub async fn role_rename(
    ctx: Context<'_>,
    #[description = "Role to rename"] role: serenity::Role,
    #[description = "New name"] new_name: String
) -> Result<(), Error> {
    debug!("Received context object {:?}.", &ctx);
    if Vec::from(consts::CANNOT_MODIFY).contains(&role.id.0) {
        let _ = ctx
            .send(|m| m.content("You cannot rename this role.").ephemeral(true))
            .await;
        Ok(())
    } else {
        create_vote(
            &ctx,
            format!("Rename role <@&{}> to {}", &role.id.0, new_name),
            VoteAction::RoleRename(RoleRename {
                role_id: role.id.0,
                new_name,
                votes: 1,
                ogmsg: 0,
                already_voted: vec![],
                finished: false,
            }),
        )
        .await
    }
}
