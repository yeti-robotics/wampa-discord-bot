mod event_handler;
mod command;

use std::env;
use dotenv::dotenv;
use serenity::Client;

fn main() {
    dotenv().ok();

    let token = env::var("DISCORD_TOKEN").expect("Discord token env var not set");
    let mut client = Client::new(&token, event_handler::Handler).expect("Error creating client");

    if let Err(err) = client.start() {
        println!("Client error: {:?}", err);
    }
}
