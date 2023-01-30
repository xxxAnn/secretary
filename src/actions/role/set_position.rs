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
pub struct RoleSetPosition {
    role_id: u64,
    position: u64,
    votes: i16,
    pub ogmsg: u64,
    pub already_voted: Vec<(u64, bool)>,
    pub finished: bool,
}

impl RoleSetPosition {
    pub fn handle(&mut self, p: i16) -> i16 {
        self.votes += p;

        self.votes
    }
    pub async fn call(self, http: impl AsRef<Http>) {
        if let Err(e) = serenity::GuildId(consts::GUILD_ID)
            .edit_role_position(&http, serenity::RoleId(self.role_id), self.position)
            .await
        {
            error!("Failed to edit role position. {:?}", e);
        }
    }
    pub fn action(self) -> VoteAction {
        VoteAction::RoleSetPosition(self)
    }
}

#[poise::command(
    slash_command,
    description_localized("en-US", "Changes the position of the role."),
    rename = "set_position"
)]
pub async fn role_set_position(
    ctx: Context<'_>,
    #[description = "Role to block from the channel"] role: serenity::Role,
    #[description = "New position of the role"] position: u64,
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
    } else {
        create_vote(
            &ctx,
            format!("Move role <@&{}> to Position({})", &role.id.0, &position),
            VoteAction::RoleSetPosition(RoleSetPosition {
                role_id: role.id.0,
                position,
                ogmsg: 0,
                votes: 0,
                already_voted: vec![],
                finished: false,
            }),
        )
        .await
    }
}
