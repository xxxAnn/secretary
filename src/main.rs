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

use std::{
    io::Write,
    sync::{Arc, Mutex},
};

use poise::{
    async_trait,
    serenity_prelude::{self as serenity, EventHandler, Http},
};

use log::{debug, error, info};

mod actions;
mod consts;

use actions::{channel, message, role, user, VoteAction};
use rand::Rng;

#[poise::command(slash_command, subcommands("message", "role", "user", "channel"))]
async fn propose(_: Context<'_>) -> Result<(), Error> {
    Ok(())
}

#[poise::command(slash_command)]
async fn sync(ctx: Context<'_>) -> Result<(), Error> {
    ctx.data().lock().unwrap().nrq = (ctx
        .guild()
        .unwrap()
        .members(&ctx, None, None)
        .await
        .unwrap()
        .iter()
        .filter(|m| {
            m.roles
                .contains(&serenity::RoleId(1_069_130_087_116_578_908))
        })
        .collect::<Vec<&serenity::Member>>()
        .len() as f32
        / 3f32)
        .ceil() as i16;
    if let Err(e) = ctx
        .say(format!(
            "Synced. The number of votes required is now {}.",
            ctx.data().lock().unwrap().nrq
        ))
        .await
    {
        error!("Error responding to sync. {:?}", e);
    }
    Ok(())
}

#[poise::command(slash_command)]
async fn declare_session_end(ctx: Context<'_>) -> Result<(), Error> {
    if ctx.author().id == 331_431_342_438_875_137 {
        let k = chrono::Utc::now() - ctx.data().lock().unwrap().started;
        let l = ctx.data().lock().unwrap().v.len();
        serenity::ChannelId(consts::VOTE_CHANNEL)
            .send_message(&ctx, |msg| {
                msg.embed(|e| {
                    e.title(format!("Session {}", consts::SESSION_NUMBER))
                        .description(format!(
                            "End of Session. 
            Session started on {} and lasted {} hour(s). Passed {} motion(s) this session.
            ",
                            ctx.data()
                                .lock()
                                .unwrap()
                                .started
                                .format("%Y-%m-%d at %H:%M:%S UTC+0"),
                            k.num_hours(),
                            l
                        ))
                        .color(ctx.data().lock().unwrap().color)
                })
            })
            .await
            .unwrap();
        if let Err(e) = ctx
            .send(|r| {
                r.content("Succesfully ended session. Shutting down bot.".to_string())
                    .ephemeral(true)
            })
            .await
        {
            error!("Error responding to end of session declaration. {:?}", &e);
            Err(Box::new(e))
        } else {
            let contents = std::fs::read_to_string("src/consts.rs").unwrap();

            std::fs::write(
                "src/consts.rs",
                contents.replace(
                    &format!(
                        "pub const SESSION_NUMBER: u16 = {};",
                        consts::SESSION_NUMBER
                    ),
                    &format!(
                        "pub const SESSION_NUMBER: u16 = {};",
                        consts::SESSION_NUMBER + 1
                    ),
                ),
            )
            .unwrap();

            std::process::abort();
        }
    } else if let Err(e) = ctx
        .send(|r| {
            r.content("You are not allowed to declare the end of a session.".to_string())
                .ephemeral(true)
        })
        .await
    {
        error!("Error responding to end of session declaration. {:?}", &e);
        Err(Box::new(e))
    } else {
        Ok(())
    }
}

#[derive(Debug)]
struct Handler {
    d: Arc<Mutex<Data>>,
}

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, ctx: serenity::Context, _: serenity::Ready) {
        serenity::ChannelId(consts::VOTE_CHANNEL)
            .send_message(&ctx, |msg| {
                msg.embed(|e| {
                    e.title(format!("Session {}", consts::SESSION_NUMBER))
                        .description(format!(
                            "The Secretary restarted. Hence, all prior votes were rendered invalid.
            
            _ _    Beginning of Session. ({})",
                            self.d
                                .lock()
                                .unwrap()
                                .started
                                .format("%Y-%m-%d %H:%M:%S UTC+0")
                        ))
                        .color(self.d.lock().unwrap().color)
                })
            })
            .await
            .unwrap();
    }
    async fn interaction_create(&self, ctx: serenity::Context, interaction: serenity::Interaction) {
        if let serenity::Interaction::MessageComponent(component) = interaction {
            // the custom_id is set as Y-{index} or N-{index},
            // here we split it and get the Y and {index} separately.
            let t = component.data.custom_id.split('-').collect::<Vec<&str>>();
            // tries to parse the index as usize but might fail.
            // (for example if the value of {index} is too high)
            let index = match t[1].parse::<usize>() {
                Ok(i) => i,
                Err(e) => {
                    error!(
                        "Failed to parse index, invalid component id ({}). {:?}",
                        component.data.custom_id, e
                    );
                    // defaulting to 0 is probably the best thing to do since 0 is likely
                    // to be finished. however we might accidentally update the wrong vote
                    // i'll fix that eventually... probably. (it probably won't happen
                    // though cause you would basically need to overflow up to 4294967296
                    // different motions).
                    0
                }
            };
            info!("Received button press for vote index {} by user {}#{} with user id {}. The vote is {} (Y/N).", 
            &index, component.user.name, component.user.discriminator, component.user.id.0, t[0]);
            debug!("Received component interaction {:?}.", component);
            let mut p: i16 = 0;
            let r = component.user.id.0;
            debug!("Checking if vote {} is finished.", &index);
            // here we check if the vote is marked as finished
            if !self.d.lock().unwrap().v[index].is_finished() {
                debug!("Vote wasn't finished. Calculating appropriate value to add to the vote.");
                // adds 1 if the vote marker is "Y" otherwise removes 1. this is the better
                // way to do it since accidentally voting against a proposition
                // is less problematic than accidentally voting for, which
                // may wrongly trigger the execution of the command in the case of an error.
                if t[0] == "Y" {
                    p += 1
                } else {
                    p -= 1
                }
                // the already voted function will return 0 if the user already voted in the same way
                // hence not doing anything. if the user already voted in the opposite way it returns
                // 2 hence cancelling their previous vote. it returns 1 if the user hasn't previously voted.
                p *= self.d.lock().unwrap().v[index].already_voted(r, t[0] == "Y");
                debug!("Adding {} to the vote.", p);
                debug!("Calculating and incrementing the vote tally.");
                // calculates the number of vote after p is added to it
                let tally = self.d.lock().unwrap().v[index].handle_tally(p);
                debug!("Tally is now {}", tally);
                debug!(
                    "Verifying if tally attained required number of votes ({}).",
                    self.d.lock().unwrap().nrq
                );
                if tally >= self.d.lock().unwrap().nrq {
                    debug!("Dummy Creating dummy to call and throw away.");
                    // creates a dummy, a clone of the VoteAction. this is so it can be thrown
                    // away with all its fields. the call function may need ownership of Self.
                    let dummy = self.d.lock().unwrap().v[index].dummy();
                    debug!("Created dummy VoteAction {:?}.", dummy);
                    debug!("Calling dummy.");
                    // calls the function with the ctx.
                    // this takes ownership of the dummy and drops it.
                    // this is very likely to make http calls to discord.
                    dummy.call(&ctx).await;
                    debug!("Setting vote as completed.");
                    // sets the actual VoteAction as finished.
                    self.d.lock().unwrap().v[index].set_finished();
                }
                if let Err(e) = component
                    .create_interaction_response(&ctx, |resp| {
                        resp.interaction_response_data(|dat| {
                            dat.content(format!(
                                "Succesfully voted. {}/{}",
                                tally,
                                self.d.lock().unwrap().nrq
                            ))
                            .ephemeral(true)
                        })
                    })
                    .await
                {
                    error!("Failed to reply to interaction. {:?}", e)
                }
            } else if let Err(e) = component
                .create_interaction_response(&ctx, |resp| {
                    resp.interaction_response_data(|dat| {
                        dat.content("This vote is over.".to_string())
                            .ephemeral(true)
                    })
                })
                .await
            {
                error!("Failed to reply to interaction. {:?}", e)
            }
        }
    }
}

#[derive(Debug)]
pub struct Data {
    v: Vec<VoteAction>,
    nrq: i16,
    started: chrono::DateTime<chrono::Utc>,
    color: (u8, u8, u8),
}
type Error = Box<dyn std::error::Error + Send + Sync>;
pub type Context<'a> = poise::Context<'a, Arc<Mutex<Data>>, Error>;

pub async fn create_vote(
    ctx: &Context<'_>,
    proposal: String,
    mut va: VoteAction,
) -> Result<(), Error> {
    let index = ctx.data().lock().unwrap().v.len();
    info!("Adding new vote action at index {}.", &index);
    debug!("{:?}", &va);

    let vote_chann = serenity::ChannelId(consts::VOTE_CHANNEL);
    debug!("Created ChannelId: {}", vote_chann);

    match vote_chann
        .send_message(&ctx, |m| {
            m.embed(|e| {
                e.title("Vote")
                    .description(format!("Proposal to {proposal}."))
                    .color(ctx.data().lock().unwrap().color)
            })
            .components(|c| {
                c.create_action_row(|r| {
                    r.create_button(|byes| {
                        byes.label("Yes")
                            .style(serenity::ButtonStyle::Success)
                            .custom_id(format!("Y-{index}"))
                    })
                    .create_button(|byes| {
                        byes.label("No")
                            .style(serenity::ButtonStyle::Danger)
                            .custom_id(format!("N-{index}"))
                    })
                })
            })
        })
        .await
    {
        Ok(m) => va.set_ogmsg(m.id.0),
        Err(e) => error!("Failed to send vote proposal. {:?}", e),
    }

    ctx.data().lock().unwrap().v.push(va);

    if let Err(e) = ctx
        .send(|f| f.content("Succesfully created proposal").ephemeral(true))
        .await
    {
        error!("Failed to reply to vote proposal command. {:?}", e)
    }

    info!("Succesfully created vote proposal with index {}.", &index);
    Ok(())
}

#[tokio::main]
async fn main() {
    let mut rng = rand::thread_rng();
    env_logger::Builder::new()
        .format(|buf, record| {
            writeln!(
                buf,
                "{} [{}] - {}",
                chrono::Utc::now().format("%Y-%m-%dT%H:%M:%S"),
                record.level(),
                record.args()
            )
        })
        .filter(Some("secretary"), consts::LEVEL)
        .init();

    let data = Arc::new(Mutex::new(Data {
        v: vec![],
        nrq: consts::DEFAULT_NRQ,
        started: chrono::Utc::now(),
        color: (
            rng.gen_range(0..=255),
            rng.gen_range(0..=255),
            rng.gen_range(0..=255),
        ),
    }));
    info!("Building framework");
    let data2 = data.clone();
    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![propose(), sync(), declare_session_end()],
            ..Default::default()
        })
        .client_settings(|c| c.event_handler(Handler { d: data }))
        .token(std::env::var("DISCORD_TOKEN").expect("missing DISCORD_TOKEN"))
        .intents(serenity::GatewayIntents::non_privileged())
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_in_guild(
                    ctx,
                    &framework.options().commands,
                    serenity::GuildId(consts::GUILD_ID),
                )
                .await?;
                Ok(data2)
            })
        });
    info!("Running framework");
    if let Err(e) = framework.run().await {
        error!("Failed running framework. {:?}", e)
    };
}
