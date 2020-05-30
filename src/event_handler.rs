use serenity::prelude::*;
use serenity::model::{ channel::Message, gateway::Ready };

use crate::command::Command;

const COMMAND_PREFIX: &str = "?";

pub struct Handler;

impl EventHandler for Handler {
    fn ready(&self, _ctx: Context, _data_about_bot: Ready) {
        println!("Successfully connected!");
    }

    fn message(&self, ctx: Context, msg: Message) {
        if !msg.content.starts_with(COMMAND_PREFIX) {
            return;
        }

        let cmd = match Command::from_str(&msg.content) {
            Ok(cmd) => cmd,
            Err(err) => {
                println!("{}", err);
                return;
            }
        };

        if let Err(err) = cmd.exec(ctx, msg) {
            println!("{}", err);
        }
    }
}

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
