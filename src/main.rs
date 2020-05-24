mod event_handler;

use std::env;
use dotenv::dotenv;
use serenity::Client;

fn main() {
    dotenv().ok();

    let token = env::var("DISCORD_TOKEN").expect("Discord token env var not set");
    let mut client = Client::new(&token, event_handler::Handler).expect("Err creating client");

    if let Err(err) = client.start() {
        println!("Client error: {:?}", err);
    }
}
