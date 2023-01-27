use crate::*;

#[derive(Debug, Clone)]
pub struct RoleDeleteAction {
    role_id: u64,
    votes: i16,
    pub ogmsg: u64,
    pub already_voted: Vec<(u64, bool)>,
    pub finished: bool
}

impl RoleDeleteAction {
    pub fn handle(&mut self, p: i16) -> i16 {
        self.votes += p;
        
        self.votes
    }
    pub async fn call(self, http: impl AsRef<Http>) {
        if let Err(e) = serenity::GuildId(consts::GUILD_ID).delete_role(&http, self.role_id).await {
            error!("Failed to delete role. {:?}", e)
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
        VoteAction::RoleDelete(self)
    }
}

#[poise::command(slash_command, rename="delete")]
pub async fn role_delete(
    ctx: Context<'_>,
    #[description = "Role to delete"]
    role: serenity::Role
) -> Result<(), Error> {    
    info!("Received command by user named {}#{} with user id {}.", ctx.author().name, ctx.author().discriminator, ctx.author().id.0);
    debug!("Received context object {:?}.", &ctx);
    create_vote(
        &ctx, 
        format!("Delete role <@&{}>", &role.id),
    VoteAction::RoleDelete( RoleDeleteAction { 
        role_id: role.id.0,
        votes: 0,
        ogmsg: 0,
        already_voted: vec![],
        finished: false})
    ).await
}