use std::env;

use yorokobot::client::{Client, ClientCredentials};

#[tokio::main]
async fn main() {
    let discord_token = env::var("DISCORD_TOKEN").expect("Cannot fetch Discord token");

    let credentials = ClientCredentials {
        discord_token: &discord_token,
    };

    let mut client = Client::new(credentials).await;

    client.connect_discord().await;
}
