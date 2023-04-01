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
#![feature(async_closure)]

use std::{
    io::Write,
    sync::{Arc, Mutex},
};

use poise::{
    async_trait,
    serenity_prelude::{self as serenity, EventHandler, Http},
};

use log::{debug, error, info};

use std::collections::BTreeMap;

mod actions;
mod consts;

use actions::{channel, message, role, user, VoteAction};
use rand::Rng;

#[poise::command(slash_command, subcommands("message", "role", "user", "channel"))]
async fn propose(_: Context<'_>) -> Result<(), Error> {
    Ok(())
}

#[poise::command(slash_command, subcommands("view_roles", "view_channel_permissions"))]
async fn view(_: Context<'_>) -> Result<(), Error> {
    Ok(())
}

pub enum Language {
    English
}

fn get_local<'a>(lang: Language, key: &'a str) -> String {
    let m: BTreeMap<String, String> = match lang {
        Language::English => serde_yaml::from_str(&std::fs::read_to_string("locals/eng.yml").unwrap()).unwrap()
    };

    return match m.get(key) {
        Some(c) => c.to_string(),
        None => "Template text".to_string()
    }
}

#[macro_export]
macro_rules! english {
    ($t:tt) => {
        get_local(Language::English, stringify!($t))
    }
}

#[poise::command(slash_command, rename = "roles")]
async fn view_roles(ctx: Context<'_>) -> Result<(), Error> {
    let guild = serenity::GuildId(consts::GUILD_ID);

    if let Ok(roles) = guild.roles(&ctx).await {
        let m = roles
            .values()
            .map(|r| {
                info!("{}", r.permissions);
                format!(
                    "{} : {} : {} : {} : {}",
                    r.name,
                    r.id,
                    if r.hoist { english!(view_roles_0_0) } else { english!(view_roles_0_1) },
                    format!("{}({})", english!(view_roles_0_2), r.position),
                    format!("{}({})", english!(view_roles_0_3), r.permissions.bits())
                )
            })
            .collect::<Vec<String>>()
            .join("\n");

        if let Err(e) = ctx.say(m).await {
            error!("{}", english!(view_roles_1));
            Err(Box::new(e))
        } else {
            Ok(())
        }
    } else {
        error!("{}", english!(view_roles_1));
        Ok(()) // <- should be an error
    }
}

#[poise::command(slash_command, rename = "channel_permissions")]
async fn view_channel_permissions(
    ctx: Context<'_>,
    c: serenity::GuildChannel,
) -> Result<(), Error> {
    let perms = c.permission_overwrites;

    let mut text = perms.iter().map(|p| {
        return match p.kind {
            serenity::PermissionOverwriteType::Role(r) => {
                format!("Role <@&{0}> {1} view the channel. Role <@&{0}> {2} send messages in the channel.",
                    r.0,
                    if p.allow.view_channel() { "can" } else { "can't" },
                    if p.allow.send_messages() { "can" } else { "can't" }
                )
            },
            _ => "".to_owned() 
        }
    }).collect::<Vec<String>>().join("\n");

    if text == "" {
        text = english!(view_channel_permissions_0).to_owned();
    }

    if let Err(e) = ctx.say(text).await {
        error!("{}", english!(view_channel_permissions_1));
        Err(Box::new(e))
    } else {
        Ok(())
    }
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
        .filter(|m| m.roles.contains(&serenity::RoleId(1069130087116578908)))
        .collect::<Vec<&serenity::Member>>()
        .len() as f32
        / 2f32)
        .ceil() as i16;
    if let Err(e) = ctx
        .say(format!(
            "{} {}.",
            english!(sync_0),
            ctx.data().lock().unwrap().nrq
        ))
        .await
    {
        error!("{} {:?}", english!(sync_1), e);
    }
    Ok(())
}

#[poise::command(slash_command)]
async fn declare_session_end(ctx: Context<'_>) -> Result<(), Error> {
    if ctx.author().id == 331431342438875137 {
        let k = chrono::Utc::now() - ctx.data().lock().unwrap().started;
        let l = ctx.data().lock().unwrap().v.len();
        serenity::ChannelId(consts::PROPOSE_CHANNEL)
            .send_message(&ctx, |msg| {
                msg.content(english!(declare_session_end_0))
            }).await.unwrap();
        serenity::ChannelId(consts::VOTE_CHANNEL)
            .send_message(&ctx, |msg| {
                msg.embed(|e| {
                    e.title(format!("Session {}", consts::SESSION_NUMBER))
                        .description(format!(
"
{} 
{} {} {} {} {} {} {}
{}{}{}
                            ",
                            english!(declare_session_end_1_0),
                            english!(declare_session_end_1_1),
                            ctx.data()
                                .lock()
                                .unwrap()
                                .started
                                .format("%Y-%m-%d at %H:%M:%S UTC+0"),
                            english!(declare_session_end_1_2),
                            k.num_hours(),
                            english!(declare_session_end_1_3),
                            l,
                            english!(declare_session_end_1_4),
                            english!(declare_session_end_1_5),
                            consts::PROPOSE_CHANNEL,
                            english!(declare_session_end_1_6)
                        ))
                        .color(ctx.data().lock().unwrap().color)
                })
            })
            .await
            .unwrap();
        if let Err(e) = serenity::GuildId(consts::GUILD_ID)
            .edit_role(
                &ctx,
                serenity::RoleId(consts::GENERAL_SECRETARY_ROLE),
                |r| {
                    r.permissions(
                        serenity::Permissions::from_bits(1071698529857).unwrap()
                            | serenity::Permissions::MODERATE_MEMBERS
                            | serenity::Permissions::MANAGE_MESSAGES,
                    )
                },
            )
            .await
        {
            error!(
                "{} {:?}",
                english!(declare_session_end_2),
                &e
            );
        }
        if let Err(e) = ctx
            .send(|r| {
                r.content(english!(declare_session_end_3))
                    .ephemeral(true)
            })
            .await
        {
            error!("{} {:?}", english!(declare_session_end_4), &e);
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
            if let Err(e) = serenity::ChannelId(consts::PROPOSE_CHANNEL)
                .create_permission(
                    &ctx,
                    &serenity::PermissionOverwrite {
                        allow: serenity::Permissions::VIEW_CHANNEL,
                        deny: serenity::Permissions::SEND_MESSAGES,
                        kind: serenity::PermissionOverwriteType::Role(serenity::RoleId(consts::VOTER_ROLE)),
                    },
                ).await
            {
                error!("{} {:?}", english!(declare_session_end_5), e)
            }
            std::process::abort();
        }
    } else if let Err(e) = ctx
        .send(|r| {
            r.content(english!(declare_session_end_6))
                .ephemeral(true)
        })
        .await
    {
        error!("{} {:?}", english!(declare_session_end_4), &e);
        Err(Box::new(e))
    } else {
        Ok(())
    }
}

#[derive(Debug)]
struct Handler {
    d: Arc<Mutex<Data>>,
}

#[macro_export]
macro_rules! vote_action {
    ($n:ident, $b:expr, $($f:ident: $t:ty),+) => {
        #[derive(Debug, Clone)]
        pub struct $n {
            $($f: $t,)+
            //
            votes: i16,
            pub ogmsg: u64,
            pub already_voted: Vec<(u64, bool)>,
            pub finished: bool,
        }

        impl $n {
            pub fn handle(&mut self, p: i16) -> i16 {
                self.votes += p;

                self.votes
            }
            #[inline(always)]
            pub async fn call(self, http: std::sync::Arc<Http>) {
                $b(self, http).await;
            }
            pub fn action(self) -> VoteAction {
                VoteAction::$n(self)
            }
            pub fn new($($f: $t,)+) -> Self {
                Self {
                    votes: 1,
                    ogmsg: 0,
                    already_voted: Vec::new(),
                    finished: false,
                    $($f,)+
                }
            }
        }
    }
}

#[macro_export]
macro_rules! generate_command {
    ($fnm:ident, $rnm:expr, $txt:expr, $g:expr, $($e:expr, $n:ident: $t:ty),*) => {
        #[poise::command(slash_command, rename=$rnm)]
        pub async fn $fnm(
            ctx: Context<'_>,
            $(#[description = $e] $n: $t,)*
        ) -> Result<(), Error> {
            create_vote(&ctx, $txt($(&$n,)*), $g($($n,)*)).await
        }
    }
}

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, ctx: serenity::Context, _: serenity::Ready) {
        serenity::ChannelId(consts::PROPOSE_CHANNEL)
            .send_message(&ctx, |msg| { 
                msg.content("Session started - channel opened") }).await.unwrap();
        let args: Vec<String> =  std::env::args().collect();
        match args.get(1) {
            Some(t) => {
                if t.to_ascii_uppercase() == "UPDATE" {
                    serenity::ChannelId(consts::INFO_CHANNEL)
                    .send_message(&ctx, |msg| {
                        msg.content(&std::fs::read_to_string("UPDATE_LOG").unwrap())
                    }).await.unwrap();
                }
            }
            _ => {}
        }
        let guild_id = serenity::GuildId(consts::GUILD_ID);
        if let Err(e) = serenity::ChannelId(consts::PROPOSE_CHANNEL)
            .create_permission(
                &ctx,
                &serenity::PermissionOverwrite {
                    allow: serenity::Permissions::VIEW_CHANNEL
                        | serenity::Permissions::SEND_MESSAGES,
                    deny: serenity::Permissions::empty(),
                    kind: serenity::PermissionOverwriteType::Role(serenity::RoleId(consts::VOTER_ROLE)),
                },
            ).await
        {
            error!("{} {:?}", english!(ready_0), e);
        }
        for m in guild_id
            .members(&ctx, None, None)
            .await
            .expect("Could not find members")
            .iter_mut()
            .filter(|m: &&mut serenity::Member| !m.communication_disabled_until.is_none())
        {
            m.enable_communication(&ctx).await.unwrap();
        }
        if let Err(e) = guild_id
            .edit_role(
                &ctx,
                serenity::RoleId(consts::GENERAL_SECRETARY_ROLE),
                |r| r.permissions(serenity::Permissions::from_bits(1071698529857).unwrap()),
            )
            .await
        {
            error!(
                "{} {:?}",
                english!(ready_1), &e
            );
        }
        serenity::ChannelId(consts::VOTE_CHANNEL)
            .send_message(&ctx, |msg| {
                msg.embed(|e| {
                    e.title(format!("Session {}", consts::SESSION_NUMBER))
                        .description(format!(
                            "{}

_ _    {} ({})",
                            english!(ready_2_0),
                            english!(ready_2_1),
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
                        "{} ({}). {:?}", english!(interaction_create_0),
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
            debug!("Received component interaction {:?}.", component);
            let mut p: i16 = 0;
            let r = component.user.id.0;
            if self.d.lock().unwrap().v.get(index).is_some() {
                debug!("Checking if vote {} is finished.", &index);
                if !self.d.lock().unwrap().v[index].is_finished() {
                    debug!(
                        "Vote wasn't finished. Calculating appropriate value to add to the vote."
                    );
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
                        let dummy = self.d.lock().unwrap().v[index].dummy();
                        debug!("Created dummy VoteAction {:?}.", dummy);
                        debug!("Calling dummy.");
                        dummy.call(&ctx).await;
                        debug!("Setting vote as completed.");
                        self.d.lock().unwrap().v[index].set_finished();
                    }
                    if let Err(e) = component
                        .create_interaction_response(&ctx, |resp| {
                            resp.interaction_response_data(|dat| {
                                dat.content(format!(
                                    "{} {}/{}",
                                    english!(interaction_create_1),
                                    tally,
                                    self.d.lock().unwrap().nrq
                                ))
                                .ephemeral(true)
                            })
                        })
                        .await
                    {
                        error!("{} {:?}", english!(interaction_create_2), e)
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
                    error!("{} {:?}", english!(interaction_create_2), e)
                }
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
    active_info!(ctx, "Adding new vote action at index {}.", &index);
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

    va.already_voted(ctx.author().id.0, true);
    let tally = va.handle_tally(0);

    ctx.data().lock().unwrap().v.push(va);
    if tally >= ctx.data().lock().unwrap().nrq {
        debug!("Dummy Creating dummy to call and throw away.");
        let dummy = ctx.data().lock().unwrap().v[index].dummy();
        debug!("Created dummy VoteAction {:?}.", dummy);
        debug!("Calling dummy.");
        dummy.call(ctx.serenity_context()).await;
        debug!("Setting vote as completed.");
        ctx.data().lock().unwrap().v[index].set_finished();
    }

    if let Err(e) = ctx
        .send(|f| f.content("Succesfully created proposal and voted for it").ephemeral(true))
        .await
    {
        error!("Failed to reply to vote proposal command. {:?}", e)
    }

    Ok(())
}

#[macro_export]
macro_rules! active_info {
    ($n:ident, $($e:expr),+) => {
        serenity::ChannelId(consts::LOG_CHANNEL)
            .send_message(&$n, |msg| {
                msg.content(format!("{} [INFO] - {}", chrono::Utc::now().format("%Y-%m-%dT%H:%M:%S"), format!($($e),+)))
            }).await.unwrap();
        info!($($e),+);
    }
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
            commands: vec![propose(), sync(), declare_session_end(), view()],
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
