use std::env;

use serenity::{async_trait, prelude::*};
use serenity::model::{ channel::Message, gateway::Ready, id::GuildId, guild::Member, id::ChannelId };

use crate::command::Command;

pub struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, _ctx: Context, _data_about_bot: Ready) {
        println!("Successfully connected!");
    }

    async fn message(&self, ctx: Context, msg: Message) {
        if !msg.content.starts_with(&env::var("COMMAND_PREFIX").unwrap()) {
            return;
        }

        let cmd = match Command::from_str(&msg.content) {
            Ok(cmd) => cmd,
            Err(err) => {
                println!("{}", err);
                return;
            }
        };

        if let Err(err) = cmd.exec(ctx, msg).await {
            println!("{}", err);
        }
    }

    async fn guild_member_addition(&self, ctx: Context, new_member: Member) {
        let welcome_channel = ChannelId::new(env::var("WELCOME_CHANNEL_ID").unwrap().parse::<u64>().unwrap());
        let welcome_msg = format!(
            "Welcome, <@{}>, to the YETI Discord! Please let us know your name by typing `?name <yourName>`. \
            For example, if your name is Wampa, you'd type `?name Wampa`. \
            Once you do that, you can head over to <#{}> to let us know what you do/want to do on the team.",
            new_member.user.id,
            env::var("ROLE_CHANNEL_ID").unwrap());
        if let Err(why) = welcome_channel.say(&ctx.http, welcome_msg).await {
            println!("Error sending message: {:?}", why);
        }
    }
}

#[derive(Debug)]
pub enum WampaError {
    InternalServerError(String),
    InvalidCmd(String),
}

impl<E: std::error::Error> From<E> for WampaError {
    fn from(e: E) -> Self {
        WampaError::InternalServerError(format!("{:?}", e))
    }
}

impl std::fmt::Display for WampaError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            WampaError::InternalServerError(msg) => write!(f, "Internal server error, contact the Slack Master: {}", msg),
            WampaError::InvalidCmd(help_text) => write!(f, "{}", help_text),
        }
    }
}
