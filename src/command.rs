use std::env;

use serenity::all::{EditMember, GetMessages};
use serenity::prelude::*;
use serenity::model::{ channel::Message, id::RoleId };

use crate::event_handler::WampaError;

pub enum Command {
    Name(String)
}

impl Command {
    pub fn from_str(s: &str) -> Result<Self, WampaError> {
        let msg = s.chars()
            .skip(1)
            .collect::<String>();
        let mut params = msg.split_whitespace();
        match params.next().unwrap() {
            "name" => Ok(Command::Name(params.collect::<Vec<&str>>().join(" "))),
            _ => Err(WampaError::InvalidCmd("Invalid command".to_string()))
        }
    }

    pub async fn exec(&self, ctx: Context, msg: Message) -> Result<(), WampaError> {
        match &self {
            Command::Name(name) => {
                println!("{:#?}", msg);
                let guild_id  = msg.guild_id.ok_or(WampaError::InternalServerError("Error finding guild id".to_string()))?;
                guild_id.edit_member(&ctx.http, msg.author.id, EditMember::new().nickname(name)).await?;
                println!("{:#?}", guild_id);

                if msg.channel_id.get() == env::var("WELCOME_CHANNEL_ID")?.parse::<u64>()? {
                    let mut msgs = msg.channel_id.messages(&ctx.http, GetMessages::new().before(msg.id)).await?
                        .iter()
                        .filter(|m| m.content.contains(&msg.author.id.to_string()) || &m.author.id == &msg.author.id)
                        .cloned()
                        .collect::<Vec<Message>>();
                    msgs.push(msg.clone());
                    msg.channel_id.delete_messages(&ctx.http, msgs).await?;
                    let roles = vec![
                        RoleId::new(env::var("MEMBER_ROLE_ID")?.parse::<u64>()?),
                        RoleId::new(env::var("ROOKIE_ROLE_ID")?.parse::<u64>()?)
                    ];
                    guild_id.edit_member(&ctx.http, msg.author.id, EditMember::new().roles(roles)).await?;
                }
            }
        }

        Ok(())
    }
}
