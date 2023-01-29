use crate::*;

#[derive(Debug, Clone)]
pub struct ChannelDelete {
    channel_id: u64,
    votes: i16,
    pub ogmsg: u64,
    pub already_voted: Vec<(u64, bool)>,
    pub finished: bool
}

impl ChannelDelete {
    pub fn handle(&mut self, p: i16) -> i16 {
        self.votes += p;
        
        self.votes
    }
    pub async fn call(self, http: impl AsRef<Http>) {
        if let Err(e) = http.as_ref().delete_channel(self.channel_id).await {
            error!("Failed to delete channel. {:?}", e)
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
        VoteAction::ChannelDelete(self)
    }
}

#[poise::command(slash_command, rename="delete")]
pub async fn channel_delete(
    ctx: Context<'_>,
    #[description = "Channel to delete"]
    channel: serenity::GuildChannel
) -> Result<(), Error> {    
    info!("Received command by user named {}#{} with user id {}.", ctx.author().name, ctx.author().discriminator, ctx.author().id.0);
    debug!("Received context object {:?}.", &ctx);
    create_vote(
        &ctx, 
        format!("Delete the <#{}> channel.", &channel.id.0),
    VoteAction::ChannelDelete( ChannelDelete { 
        channel_id: channel.id.0,
        votes: 0,
        ogmsg: 0,
        already_voted: vec![],
        finished: false})
    ).await
}