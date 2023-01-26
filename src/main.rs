use std::{sync::{Mutex, Arc}};

use poise::{serenity_prelude::{self as serenity, EventHandler, Http}, async_trait};

mod consts;
mod actions;

use actions::message::*;

struct Handler {
    d: Arc<Mutex<Data>>
}

// todo: handle done and already voted

#[async_trait]
impl EventHandler for Handler {
    async fn interaction_create(&self, ctx: serenity::Context, interaction: serenity::Interaction) {
        if let serenity::Interaction::MessageComponent(component) = interaction {
            let t = component.data.custom_id.split("-").collect::<Vec<&str>>();
            let index = t[1].parse::<usize>().unwrap();
            let mut p: i16 = 0;
            let r = component.user.id.0;
            if !self.d.lock().unwrap().v[index].is_finished() {
                if t[0] == "Y" { p += 1 } else { p -= 1 }
                p *= self.d.lock().unwrap().v[index].already_voted(r, t[0] == "Y");
                let tally = self.d.lock().unwrap().v[index].handle_tally(p);
                if tally >= consts::NUMBER_REQUIRED {
                    let dummy = self.d.lock().unwrap().v[index].dummy();

                    dummy.call(&ctx).await;
                    self.d.lock().unwrap().v[index].set_finished();
                }
                component.create_interaction_response(&ctx, |resp| resp
                    .interaction_response_data(|dat| dat
                        .content(format!("Succesfully voted. {}/{}", tally, consts::NUMBER_REQUIRED))
                        .ephemeral(true)
                    )
                ).await.unwrap();
            } else {
                component.create_interaction_response(&ctx, |resp| resp
                    .interaction_response_data(|dat| dat
                        .content(format!("This vote is over."))
                        .ephemeral(true)
                    )
                ).await.unwrap();
            }
        }
    }
}

#[derive(Debug)]
pub enum VoteAction {
    MessageCreate(MessageCreateAction)
}

impl VoteAction {
    pub fn handle_tally(&mut self, p: i16) -> i16 {
        match self {
            VoteAction::MessageCreate(ref mut m) => {
                m.handle(p)
            }
        }
    }
    pub fn dummy(&self) -> Self {
        return match self {
            VoteAction::MessageCreate(m) => VoteAction::MessageCreate(m.clone())
        }
    }
    pub async fn call(self, http: impl AsRef<Http>) {
        match self {
            VoteAction::MessageCreate(m) => {
                m.call(http).await;
            }
        }
    }
    pub fn is_finished(&self) -> bool {
        match &self {
            VoteAction::MessageCreate(m) => m.finished
        }
    }
    pub fn set_finished(&mut self) {
        match self {
            VoteAction::MessageCreate(ref mut m) => m.finished = true,
        }
    }
    pub fn already_voted(&mut self, r: u64, vote: bool) -> i16 {
        let already_voted;
        match self {
            VoteAction::MessageCreate(ref mut m) => {
                already_voted = &mut m.already_voted;
            }
        }
        let mut factor = 1;
        let k = already_voted.contains(&(r, vote));
        if k { return 0 }

        // Removes a possible different vote
        if let Some(index) = already_voted.iter().position(|x| x == &(r, !vote)) {
            already_voted.remove(index);
            factor = 2;
        }

        already_voted.push((r, vote));

        factor
    }
}


pub struct Data {
    v: Vec<VoteAction>
}
type Error = Box<dyn std::error::Error + Send + Sync>;
pub type Context<'a> = poise::Context<'a, Arc<Mutex<Data>>, Error>;

/// Displays your or another user's account creation date
#[poise::command(slash_command, subcommands("message"))]
async fn propose(_: Context<'_>) -> Result<(), Error> {
    Ok(())
}

pub async fn create_vote(ctx: &Context<'_>, proposal: String, va: VoteAction) -> Result<(), Error> {
    let index = ctx.data().lock().unwrap().v.len();
    ctx.data().lock().unwrap().v.push(va);

    let vote_chann = serenity::ChannelId(consts::VOTE_CHANNEL);

    vote_chann.send_message(&ctx, |m| m
        .embed(|e| e.title("Vote").description(format!("Proposal to {}.", proposal)).colour((200, 12, 12)))
        .components(|c| c
            .create_action_row(|r| r
                .create_button(|byes| byes.label("Yes").style(serenity::ButtonStyle::Success).custom_id(format!("Y-{}", index)))
                .create_button(|byes| byes.label("No").style(serenity::ButtonStyle::Danger).custom_id(format!("N-{}", index)))
            )
    )).await.unwrap();

    ctx.send(|f| f
        .content("Succesfully created proposal")
        .ephemeral(true)
    ).await.unwrap();

    Ok(())
}

#[tokio::main]
async fn main() {
    let data = Arc::new(Mutex::new( Data {
        v: vec![]
    }));
    let data2 = data.clone();
    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![propose()],
            ..Default::default()
        })
        .client_settings(|c| c.event_handler(Handler {
            d: data
        }))
        .token(std::env::var("DISCORD_TOKEN").expect("missing DISCORD_TOKEN"))
        .intents(serenity::GatewayIntents::non_privileged())
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_in_guild(ctx, &framework.options().commands, serenity::GuildId(consts::GUILD_ID)).await?;
                Ok(data2)
            })
        });

    framework.run().await.unwrap();
}