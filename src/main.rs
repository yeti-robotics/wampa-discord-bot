mod event_handler;
mod command;

use std::env;
use dotenv::dotenv;
use serenity::{all::GatewayIntents, Client};

#[tokio::main]
async fn main() {
    dotenv().ok();

    let token = env::var("DISCORD_TOKEN").expect("Discord token env var not set");
    let intents = GatewayIntents::GUILD_MEMBERS | GatewayIntents::GUILDS | GatewayIntents::GUILD_MESSAGES | GatewayIntents::MESSAGE_CONTENT;
    let mut client = Client::builder(&token, intents).event_handler(event_handler::Handler).await.expect("Error creating client");

    if let Err(err) = client.start().await {
        println!("Client error: {:?}", err);
    }
}
