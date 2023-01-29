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

use crate::*;

#[derive(Debug, Clone)]
pub struct CategoryChannelCreate {
    name: String,
    votes: i16,
    pub ogmsg: u64,
    pub already_voted: Vec<(u64, bool)>,
    pub finished: bool
}

impl CategoryChannelCreate {
    pub fn handle(&mut self, p: i16) -> i16 {
        self.votes += p;
        
        self.votes
    }
    pub async fn call(self, http: impl AsRef<Http>) {

        if let Err(e) = serenity::GuildId(consts::GUILD_ID).create_channel(&http, |c| c
            .name(self.name)
            .kind(serenity::ChannelType::Category)
        ).await {
            error!("Failed to create role. {:?}", e)
        } else {
            if let Err(e) = serenity::ChannelId(consts::VOTE_CHANNEL).send_message(&http, |msg| msg
                .content("Vote passed.")
                .reference_message((serenity::ChannelId(consts::VOTE_CHANNEL), serenity::MessageId(self.ogmsg)))).await 
            {
                error!("Failed to announce vote success. {:?}", e)
            }
        }
        
    }
    pub fn action(self) -> VoteAction {
        VoteAction::CategoryChannelCreate(self)
    }
}

#[poise::command(slash_command)]
pub async fn create_category(
    ctx: Context<'_>,
    #[description = "Name of the channel to create"]
    name: String,
) -> Result<(), Error> {    
    info!("Received command by user named {}#{} with user id {}.", ctx.author().name, ctx.author().discriminator, ctx.author().id.0);
    debug!("Received context object {:?}.", &ctx);

    create_vote(
        &ctx, 
        format!("Create channel called {}", &name),
    VoteAction::CategoryChannelCreate( CategoryChannelCreate { 
        name: name,
        ogmsg: 0,
        votes: 0,
        already_voted: vec![],
        finished: false})
    ).await
}