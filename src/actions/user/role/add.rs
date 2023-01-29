use crate::*;

#[derive(Debug, Clone)]
pub struct UserRoleAdd {
    member_id: u64,
    role_id: u64,
    //
    votes: i16,
    pub ogmsg: u64,
    pub already_voted: Vec<(u64, bool)>,
    pub finished: bool,
}

impl UserRoleAdd {
    pub fn handle(&mut self, p: i16) -> i16 {
        self.votes += p;

        self.votes
    }
    pub async fn call(self, http: impl AsRef<Http>) {
        let mut user = http
            .as_ref()
            .get_member(consts::GUILD_ID, self.member_id)
            .await
            .unwrap();

        if let Err(e) = user.add_role(&http, self.role_id).await {
            error!("Failed to add role to user. {:?}", e)
        } else if let Err(e) = serenity::ChannelId(consts::VOTE_CHANNEL)
            .send_message(&http, |msg| {
                msg.content("Vote passed.").reference_message((
                    serenity::ChannelId(consts::VOTE_CHANNEL),
                    serenity::MessageId(self.ogmsg),
                ))
            })
            .await
        {
            error!("Failed to announce vote success. {:?}", e)
        }
    }
    pub fn action(self) -> VoteAction {
        VoteAction::UserRoleAdd(self)
    }
}

#[poise::command(slash_command)]
pub async fn role_add(
    ctx: Context<'_>,
    #[description = "The user to add the role to"] member: serenity::Member,
    #[description = "The role to add to the user"] role: serenity::Role,
) -> Result<(), Error> {
    info!(
        "Received command by user named {}#{} with user id {}.",
        ctx.author().name,
        ctx.author().discriminator,
        ctx.author().id.0
    );
    debug!("Received a member object {:?}.", &member);
    debug!("Received context object {:?}.", &ctx);
    create_vote(
        &ctx,
        format!(
            "Add role <@&{}> to user <@{}>",
            &role.id.0, &member.user.id.0
        ),
        VoteAction::UserRoleAdd(UserRoleAdd {
            member_id: member.user.id.0,
            role_id: role.id.0,
            votes: 0,
            ogmsg: 0,
            already_voted: vec![],
            finished: false,
        }),
    )
    .await
}
