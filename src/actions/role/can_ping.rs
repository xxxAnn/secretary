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
    UserCanPing,
    move |me: UserCanPing, http: std::sync::Arc<Http>| {
        async move {
            let p = me.can_ping;
            if let Err(e) = serenity::GuildId(consts::GUILD_ID).edit_role(&http, me.role_id, move |r| 
                if p {
                    r.permissions(serenity::Permissions::from_bits(1071698529857).unwrap() | serenity::Permissions::MENTION_EVERYONE)
                } else {
                    r.permissions(serenity::Permissions::from_bits(1071698529857).unwrap())
                }
                
            ).await {
                error!("Failed to edit role permissions. {:?}", e)
                
            } else {
                if let Err(e) = serenity::ChannelId(consts::VOTE_CHANNEL)
                .send_message(&http, |msg| {
                    msg.content("Vote passed.").reference_message((
                        serenity::ChannelId(consts::VOTE_CHANNEL),
                        serenity::MessageId(me.ogmsg),
                    ))
                })
                .await {
                    error!("Failed to announce vote success. {:?}", e)
                }
            }
        }
    },
    role_id: u64,
    can_ping: bool
);

generate_command!(
    can_ping_everyone,
    "can_ping_everyone",
    |r: &serenity::Role| -> String {
        format!("Make role <@&{}> able to ping everyone", &r.id.0)
    },
    |r: serenity::Role| -> VoteAction {
        UserCanPing::new(r.id.0, true).action()
    },
    "The role that should be able to ping everyone",
    role: serenity::Role
);

generate_command!(
    cant_ping_everyone,
    "cant_ping_everyone",
    |r: &serenity::Role| -> String {
        format!("Make role <@&{}> unable to ping everyone", &r.id.0)
    },
    |r: serenity::Role| -> VoteAction {
        UserCanPing::new(r.id.0, false).action()
    },
    "The role that should be able to ping everyone",
    role: serenity::Role
);
