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
    consts, create_vote, debug, error, info, serenity, vote_action, Context, Error, Http,
    VoteAction,
};

vote_action!(
    ChannelPurge,
    move |me: ChannelPurge, http: std::sync::Arc<Http>| {
        async move {
            let cid = serenity::ChannelId(me.channel_id);
            let mids = cid.messages(&http, |ms| ms.limit(me.limit)).await.unwrap();
            if let Err(e) = cid.delete_messages(&http, mids).await {
                error!("Failed to purge channel. {:?}", e)
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
    limit: u64
);

#[poise::command(
    slash_command,
    rename = "purge",
    description_localized("en-US", "Deletes the last 100 messages in the channel.")
)]
pub async fn channel_purge(
    ctx: Context<'_>,
    #[description = "Channel in which to purge messages"] channel: serenity::GuildChannel,
    #[description = "Amount of messages to purge"] limit: u64,
) -> Result<(), Error> {
    info!(
        "Received command by user named {}#{} with user id {}.",
        ctx.author().name,
        ctx.author().discriminator,
        ctx.author().id.0
    );
    debug!("Received context object {:?}.", &ctx);
    if Vec::from(consts::CANNOT_MODIFY_CHANNEL).contains(&channel.id.0) {
        let _ = ctx
            .send(|m| {
                m.content("You cannot purge from these channels.")
                    .ephemeral(true)
            })
            .await;
        Ok(())
    } else {
        create_vote(
            &ctx,
            format!("Purge 100 messages from the <#{}> channel", &channel.id.0),
            VoteAction::ChannelPurge(ChannelPurge {
                channel_id: channel.id.0,
                votes: 1,
                limit,
                ogmsg: 0,
                already_voted: vec![],
                finished: false,
            }),
        )
        .await
    }
}
