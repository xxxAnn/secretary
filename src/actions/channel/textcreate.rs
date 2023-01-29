use crate::*;

#[derive(Debug, Clone)]
pub struct TextChannelCreate {
    name: String,
    category_id: Option<u64>,
    topic: String,
    votes: i16,
    pub ogmsg: u64,
    pub already_voted: Vec<(u64, bool)>,
    pub finished: bool
}

impl TextChannelCreate {
    pub fn handle(&mut self, p: i16) -> i16 {
        self.votes += p;
        
        self.votes
    }
    pub async fn call(self, http: impl AsRef<Http>) {
        match self.category_id {
            Some(cat_id) => {
                if let Err(e) = serenity::GuildId(consts::GUILD_ID).create_channel(&http, |c| c
                    .name(self.name)
                    .category(cat_id)
                    .kind(serenity::ChannelType::Text)
                    .topic(self.topic)
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
            None => {
                if let Err(e) = serenity::GuildId(consts::GUILD_ID).create_channel(&http, |c| c
                    .name(self.name)
                    .kind(serenity::ChannelType::Text)
                    .topic(self.topic)
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
        }
        
    }
    pub fn action(self) -> VoteAction {
        VoteAction::TextChannelCreate(self)
    }
}

#[poise::command(slash_command)]
pub async fn create_text(
    ctx: Context<'_>,
    #[description = "Name of the channel to create"]
    name: String,
    #[description = "Category of the channel"]
    category: Option<serenity::Channel>,
    #[description = "Channel topic"]
    topic: String,
) -> Result<(), Error> {    
    info!("Received command by user named {}#{} with user id {}.", ctx.author().name, ctx.author().discriminator, ctx.author().id.0);
    debug!("Received context object {:?}.", &ctx);
    let category_id = match category {
        Some(c) => Some(c.id().0),
        None => None
    };
    create_vote(
        &ctx, 
        format!("Create channel called {}", &name),
    VoteAction::TextChannelCreate( TextChannelCreate { 
        name: name,
        category_id,
        topic,
        ogmsg: 0,
        votes: 0,
        already_voted: vec![],
        finished: false})
    ).await
}