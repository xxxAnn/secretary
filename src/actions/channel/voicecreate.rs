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

use crate::{consts, create_vote, error, serenity, vote_action, Context, Error, Http, VoteAction};

vote_action!(
    VoiceChannelCreate,
    move |me: VoiceChannelCreate, http: std::sync::Arc<Http>| {
        async move {
            match me.category_id {
                Some(cat_id) => {
                    if let Err(e) = serenity::GuildId(consts::GUILD_ID)
                        .create_channel(&http, |c| {
                            c.name(me.name)
                                .category(cat_id)
                                .kind(serenity::ChannelType::Voice)
                        })
                        .await
                    {
                        error!("Failed to create role. {:?}", e)
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
                None => {
                    if let Err(e) = serenity::GuildId(consts::GUILD_ID)
                        .create_channel(&http, |c| {
                            c.name(me.name).kind(serenity::ChannelType::Voice)
                        })
                        .await
                    {
                        error!("Failed to create role. {:?}", e)
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
            }
        }
    },
    name: String,
    category_id: Option<u64>
);

#[poise::command(slash_command, rename = "create_voice")]
pub async fn create_voice(
    ctx: Context<'_>,
    #[description = "Name of the channel to create"] name: String,
    #[description = "Category of the channel"] category: Option<serenity::Channel>,
) -> Result<(), Error> {
    create_vote(
        &ctx,
        format!("Create channel called {}", name),
        VoiceChannelCreate::new(name, category.map(|c| c.id().0)).action(),
    )
    .await
}
