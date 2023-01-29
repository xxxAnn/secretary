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
pub struct ChannelPurge {
    channel_id: u64,
    limit: u64,
    votes: i16,
    pub ogmsg: u64,
    pub already_voted: Vec<(u64, bool)>,
    pub finished: bool,
}

impl ChannelPurge {
    pub fn handle(&mut self, p: i16) -> i16 {
        self.votes += p;

        self.votes
    }
    pub async fn call(self, http: impl AsRef<Http>) {
        let cid = serenity::ChannelId(self.channel_id);
        let mids = cid
            .messages(&http, |ms| ms.limit(self.limit))
            .await
            .unwrap();
        if let Err(e) = cid.delete_messages(&http, mids).await {
            error!("Failed to purge channel. {:?}", e)
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
        VoteAction::ChannelPurge(self)
    }
}

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
    if vec![969_016_622_402_650_112, 970_108_683_746_951_178].contains(&channel.id.0) {
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
                votes: 0,
                limit,
                ogmsg: 0,
                already_voted: vec![],
                finished: false,
            }),
        )
        .await
    }
}
