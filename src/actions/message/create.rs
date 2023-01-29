use crate::*;

#[derive(Debug, Clone)]
pub struct MessageCreate {
    text: String,
    channel_id: u64,
    votes: i16,
    pub ogmsg: u64,
    pub already_voted: Vec<(u64, bool)>,
    pub finished: bool
}

impl MessageCreate {
    pub fn handle(&mut self, p: i16) -> i16 {
        self.votes += p;
        
        self.votes
    }
    pub async fn call(self, http: impl AsRef<Http>) {
        let chann = serenity::ChannelId(self.channel_id);

        if let Err(e) = chann.say(&http, self.text).await {
            error!("Failed to delete messages. {:?}", e)
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
        VoteAction::MessageCreate(self)
    }
}

#[poise::command(slash_command)]
pub async fn create(
    ctx: Context<'_>,
    #[description = "Channel to send the message in"]
    channel: serenity::GuildChannel,
    #[description = "Message to send"]
    message: String
) -> Result<(), Error> {    
    info!("Received command by user named {}#{} with user id {}.", ctx.author().name, ctx.author().discriminator, ctx.author().id.0);
    debug!("Received channel object {:?}.", &channel);
    debug!("Received context object {:?}.", &ctx);
    create_vote(
        &ctx, 
        format!("Send message \"{}\" to the <#{}> channel", &message, &channel.id.0),
    VoteAction::MessageCreate( MessageCreate { 
        text: message.clone(),
        channel_id: channel.id.0,
        ogmsg: 0,
        votes: 0,
        already_voted: vec![],
        finished: false})
    ).await
}