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
    CategoryChannelCreate,
    move |me: CategoryChannelCreate, http: std::sync::Arc<Http>| {
        async move {
            if let Err(e) = serenity::GuildId(consts::GUILD_ID)
                .create_channel(&http, |c| {
                    c.name(me.name).kind(serenity::ChannelType::Category)
                })
                .await
            {
                error!("Failed to create category. {:?}", e)
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
    name: String
);

generate_command!(
    create_category,
    "create_category",
    |n: &str| -> String { format!("Create channel called {}", n) },
    |n| -> VoteAction { CategoryChannelCreate::new(n).action() },
    "Name of the channel to create",
    name: String
);
