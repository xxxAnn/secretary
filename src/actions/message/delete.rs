use crate::*;

#[derive(Debug, Clone)]
pub struct MessageDeleteAction {
    channel_id: u64,
    message_id: u64,
    votes: i16,
    pub ogmsg: u64,
    pub already_voted: Vec<(u64, bool)>,
    pub finished: bool
}

impl MessageDeleteAction {
    pub fn handle(&mut self, p: i16) -> i16 {
        self.votes += p;
        
        self.votes
    }
    pub async fn call(self, http: impl AsRef<Http>) {
        let chann = serenity::ChannelId(self.channel_id);

        if let Err(e) = chann.delete_message(&http, self.message_id).await {
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
        VoteAction::MessageDelete(self)
    }
}

#[poise::command(slash_command)]
pub async fn delete(
    ctx: Context<'_>,
    #[description = "Channel in which the message"]
    channel: serenity::GuildChannel,
    #[description = "ID of the message to delete"]
    message_id: String,
) -> Result<(), Error> {    
    info!("Received command by user named {}#{} with user id {}.", ctx.author().name, ctx.author().discriminator, ctx.author().id.0);
    debug!("Received channel object {:?}.", &channel);
    debug!("Received context object {:?}.", &ctx);

    let parsed_id = match message_id.parse::<u64>() {
        Ok(m) =>  m,
        Err(e) => {
            error!("{:?}", e);
            0u64
        } 
    };

    create_vote(
        &ctx, 
        format!("Delete this message https://discord.com/channels/{}/{}/{}", &channel.guild_id.0, &channel.id.0, &message_id),
    VoteAction::MessageDelete( MessageDeleteAction { 
        message_id: parsed_id,
        channel_id: channel.id.0,
        ogmsg: 0,
        votes: 0,
        already_voted: vec![],
        finished: false})
    ).await
}