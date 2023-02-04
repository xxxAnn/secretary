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
    UserRoleAdd,
    move |me: UserRoleAdd, http: std::sync::Arc<Http>| {
        async move {
            let mut user = (&http)
                .get_member(consts::GUILD_ID, me.member_id)
                .await
                .unwrap();

            if let Err(e) = user.add_role(&http, me.role_id).await {
                error!("Failed to add role to user. {:?}", e)
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
    member_id: u64,
    role_id: u64
);

generate_command!(
    role_add,
    "role_add",
    |m: &serenity::Member, r: &serenity::Role| -> String {
        format!("Add role <@&{}> to user <@{}>", &r.id.0, &m.user.id.0)
    },
    |m: serenity::Member, r: serenity::Role| -> VoteAction {
        UserRoleAdd::new(m.user.id.0, r.id.0).action()
    },
    "The user to add the role to",
    member: serenity::Member,
    "The role to add to the user",
    role: serenity::Role
);
