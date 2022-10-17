use std::env;

use yorokobot::client::{Client, ClientOptions};

#[tokio::main]
async fn main() {
    let discord_token = env::var("DISCORD_TOKEN").expect("Cannot fetch Discord token");

    let options = ClientOptions {
        discord_token: &discord_token,
    };

    let mut client = Client::new(options).await;

    client.connect_discord().await;
}
