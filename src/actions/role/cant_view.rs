use crate::*;

#[derive(Debug, Clone)]
pub struct CantView {
    channel_id: u64,
    role_id: u64,
    votes: i16,
    pub ogmsg: u64,
    pub already_voted: Vec<(u64, bool)>,
    pub finished: bool
}

impl CantView {
    pub fn handle(&mut self, p: i16) -> i16 {
        self.votes += p;
        
        self.votes
    }
    pub async fn call(self, http: impl AsRef<Http>) {
        if let Err(e) = serenity::ChannelId(self.channel_id).create_permission(&http, 
            &serenity::PermissionOverwrite { 
                allow: serenity::Permissions::empty(), 
                deny: serenity::Permissions::VIEW_CHANNEL, 
                kind: serenity::PermissionOverwriteType::Role(serenity::RoleId(self.role_id)) 
            }
        ).await {
            error!("Failed to edit role permissions. {:?}", e)
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
        VoteAction::CantView(self)
    }
}

#[poise::command(slash_command, description_localized("en-US", "Prevents role from viewing the selected channel."))]
pub async fn cant_view(
    ctx: Context<'_>,
    #[description = "Role to block from the channel"]
    role: serenity::Role,
    #[description = "Channel to block the role from seeing"]
    channel: serenity::GuildChannel
) -> Result<(), Error> {    
    info!("Received command by user named {}#{} with user id {}.", ctx.author().name, ctx.author().discriminator, ctx.author().id.0);
    debug!("Received context object {:?}.", &ctx);
    create_vote(
        &ctx, 
        format!("Block role <@&{}> from seeing <#{}>", &role.id.0, &channel.id.0),
    VoteAction::CantView( CantView { 
        channel_id: channel.id.0,
        role_id: role.id.0,
        ogmsg: 0,
        votes: 0,
        already_voted: vec![],
        finished: false})
    ).await
}