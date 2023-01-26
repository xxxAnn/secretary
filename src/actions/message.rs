use crate::*;

#[derive(Debug, Clone)]
pub struct MessageCreateAction {
    text: String,
    channel_id: u64,
    votes: i16,
    pub already_voted: Vec<(u64, bool)>,
    pub finished: bool
}

impl MessageCreateAction {
    pub fn handle(&mut self, p: i16) -> i16 {
        self.votes += p;
        
        self.votes
    }
    pub async fn call(self, http: impl AsRef<Http>) {
        let chann = serenity::ChannelId(self.channel_id);

        chann.say(http, self.text).await.unwrap();
    }
}

unsafe impl Send for MessageCreateAction {}

#[poise::command(slash_command, subcommands("create"))]
pub async fn message(_: Context<'_>) -> Result<(), Error> {
    Ok(())
}

#[poise::command(slash_command)]
pub async fn create(
    ctx: Context<'_>,
    #[description = "Channel to send the message in"]
    channel: serenity::GuildChannel,
    #[description = "Message to send"]
    message: String) -> Result<(), Error> {
    // Create message
    println!("{:?}", ctx.data().lock().unwrap().v);
    
    create_vote(&ctx, format!("Send message \"{}\" to the <#{}> channel", &message, &channel.id.0),
    VoteAction::MessageCreate( MessageCreateAction { 
        text: message.clone(),
        channel_id: channel.id.0,
        votes: 0,
        already_voted: vec![],
        finished: false})).await
}