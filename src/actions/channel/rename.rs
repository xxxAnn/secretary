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

use crate::{
    consts, create_vote, error, generate_command, serenity, vote_action, Context, Error, Http,
    VoteAction,
};

vote_action!(
    GuildChannelRename,
    move |me: GuildChannelRename, http: std::sync::Arc<Http>| {
        async move {
            if let Err(e) = serenity::ChannelId(me.channel_id)
            .edit(&http, |e| {
                e.name(me.new_name)
            }).await {
                error!("Failed to rename channel. {:?}", e)
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
    channel_id: u64,
    new_name: String
);

#[poise::command(slash_command, rename = "rename")]
pub async fn channel_rename(
    ctx: Context<'_>,
    #[description = "Channel to rename"] channel: serenity::Channel,
    #[description = "New name for the channel"] new_name: String
) -> Result<(), Error> {
    if Vec::from(consts::CANNOT_MODIFY_CHANNEL).contains(&channel.id().0) {
        let _ = ctx
            .send(|m| {
                m.content("You cannot modify these channels.")
                    .ephemeral(true)
            })
            .await;
        Ok(())
    } else { 
        create_vote(
        &ctx,
        format!("Rename channel <#{}> to {}", channel.id().0, new_name),
        GuildChannelRename::new(channel.id().0, new_name).action(),
        ).await
    }      
}
