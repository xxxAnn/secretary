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
    UserRoleRemove,
    move |me: UserRoleRemove, http: std::sync::Arc<Http>| {
        async move {
            let mut user = http
                .as_ref()
                .get_member(consts::GUILD_ID, me.member_id)
                .await
                .unwrap();

            if let Err(e) = user.remove_role(&http, me.role_id).await {
                error!("Failed to remove role from user. {:?}", e);
            } else if let Err(e) = serenity::ChannelId(consts::VOTE_CHANNEL)
                .send_message(&http, |msg| {
                    msg.content("Vote passed.").reference_message((
                        serenity::ChannelId(consts::VOTE_CHANNEL),
                        serenity::MessageId(me.ogmsg),
                    ))
                })
                .await
            {
                error!("Failed to announce vote success. {:?}", e);
            }
        }
    },
    member_id: u64,
    role_id: u64
);

generate_command!(
    role_remove,
    "role_remove",
    |m: &serenity::Member, r: &serenity::Role| -> String {
        format!("Remove role <@&{}> from user <@{}>", &r.id.0, &m.user.id.0)
    },
    |m: serenity::Member, r: serenity::Role| -> VoteAction {
        UserRoleRemove::new(m.user.id.0, r.id.0).action()
    },
    "The user to remove the role from",
    member: serenity::Member,
    "The role to remove from the user",
    role: serenity::Role
);
