use serenity::prelude::*;
use serenity::model::{ channel::Message };

pub struct Handler;

impl EventHandler for Handler {
    fn message(&self, ctx: Context, msg: Message) {
        let guild_id = if let Some(guild_id) = msg.guild_id {
            guild_id
        } else {
            println!("Error getting guild id");
            return;
        };
        let guild = match ctx.cache.read().guild(guild_id) {
            Some(guild) => guild,
            None => return,
        };
        guild.read().edit_member(ctx.http, msg.author.id, |m| m.nickname(msg.content));
    }
}
