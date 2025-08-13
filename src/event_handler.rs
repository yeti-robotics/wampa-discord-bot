use std::env;

use serenity::{async_trait, prelude::*};
use serenity::model::{ channel::Message, gateway::Ready, guild::Member, id::ChannelId };

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
        let raw_msg = env::var("WELCOME_MESSAGE").unwrap_or_else(|_| "Welcome, <@USER_ID>, to the YETI Discord! Please let us know your first and last name by typing `?name yourName`. For example, if your name is Wampa Robotson, you'd type `?name Wampa Robotson`. Once you do that, you can head over to <#ROLE_CHANNEL_ID> to let us know what you do/want to do on the team.".to_string());
        let welcome_msg = raw_msg
            .replace("<@USER_ID>", &format!("<@{}>", new_member.user.id))
            .replace("<#ROLE_CHANNEL_ID>", &format!("<#{}>", env::var("ROLE_CHANNEL_ID").unwrap()))
            .replace("\\n", "\n");
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
            WampaError::InternalServerError(msg) => write!(f, "Internal server error: {}", msg),
            WampaError::InvalidCmd(help_text) => write!(f, "{}", help_text),
        }
    }
}
