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
pub struct RoleHoist {
    role_id: u64,
    hoist: bool,
    votes: i16,
    pub ogmsg: u64,
    pub already_voted: Vec<(u64, bool)>,
    pub finished: bool,
}

impl RoleHoist {
    pub fn handle(&mut self, p: i16) -> i16 {
        self.votes += p;

        self.votes
    }
    pub async fn call(self, http: impl AsRef<Http>) {
        if let Err(e) = serenity::GuildId(consts::GUILD_ID)
            .edit_role(&http, serenity::RoleId(self.role_id), |r| r
                .hoist(self.hoist)
            )
            .await
        {
            error!("Failed to edit role position. {:?}", e);
        }
    }
    pub fn action(self) -> VoteAction {
        VoteAction::RoleHoist(self)
    }
}

#[poise::command(
    slash_command,
    description_localized(
        "en-US",
        "Makes the role visible in the user bar."
    ),
    rename="hoist"
)]
pub async fn role_hoist(
    ctx: Context<'_>,
    #[description = "Role to hoist"] role: serenity::Role,
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
            format!(
                "Proposal to hoist role <@&{}>",
                &role.id.0
            ),
            VoteAction::RoleHoist(RoleHoist {
                role_id: role.id.0,
                hoist: true,
                ogmsg: 0,
                votes: 0,
                already_voted: vec![],
                finished: false,
            }),
        )
        .await
    }
}

#[poise::command(
    slash_command,
    description_localized(
        "en-US",
        "Removes the role from the user bar."
    ),
    rename="unhoist"
)]
pub async fn role_unhoist(
    ctx: Context<'_>,
    #[description = "Role to unhoist"] role: serenity::Role,
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
            format!(
                "Proposal to unhoist role <@&{}>",
                &role.id.0
            ),
            VoteAction::RoleHoist(RoleHoist {
                role_id: role.id.0,
                hoist: false,
                ogmsg: 0,
                votes: 0,
                already_voted: vec![],
                finished: false,
            }),
        )
        .await
    }
}
