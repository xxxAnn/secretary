use crate::*;

#[derive(Debug, Clone)]
pub struct RoleCreate {
    colour: u64,
    name: String,
    position: u8,
    votes: i16,
    pub ogmsg: u64,
    pub already_voted: Vec<(u64, bool)>,
    pub finished: bool
}

impl RoleCreate {
    pub fn handle(&mut self, p: i16) -> i16 {
        self.votes += p;
        
        self.votes
    }
    pub async fn call(self, http: impl AsRef<Http>) {
        if let Err(e) = serenity::GuildId(consts::GUILD_ID).create_role(&http,|r| r
            .colour(self.colour)
            .name(self.name)
            .position(self.position)
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
        VoteAction::RoleCreate(self)
    }
}

#[poise::command(slash_command, rename="create")]
pub async fn role_create(
    ctx: Context<'_>,
    #[description = "Red (Rgb)"]
    r: u8,
    #[description = "Green (rGb)"]
    g: u8,
    #[description = "Blue (rgB)"]
    b: u8,
    #[description = "Name of the role"]
    name: String,
    #[description = "Position of the role"]
    position: u8
) -> Result<(), Error> {    
    info!("Received command by user named {}#{} with user id {}.", ctx.author().name, ctx.author().discriminator, ctx.author().id.0);
    debug!("Received context object {:?}.", &ctx);
    create_vote(
        &ctx, 
        format!("Create role called {}", &name),
    VoteAction::RoleCreate( RoleCreate { 
        name,
        colour: serenity::Colour::from_rgb(r, g, b).0 as u64,
        position,
        ogmsg: 0,
        votes: 0,
        already_voted: vec![],
        finished: false})
    ).await
}