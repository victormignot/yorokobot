use std::env;

use yorokobot::{
    client::{Client, ClientCredentials},
    errors::ClientsError,
};

#[tokio::main]
async fn main() {
    let discord_token = env::var("DISCORD_TOKEN").expect("Cannot fetch Discord token");

    let mongodb_uri = env::var("MONGODB_URI").expect("Cannot fetch Mongo URI");

    let credentials = ClientCredentials {
        discord_token: &discord_token,
        mongo_uri: &mongodb_uri,
    };

    let mut client = Client::new(credentials)
        .await
        .expect("Could not create client");

    client.connect().await.unwrap_or_else(|error| match error {
        ClientsError::Database(e) => panic!("Could not connect to database: {:?}", e),
        ClientsError::Discord(e) => panic!("Could not connect to Discord: {:?}", e),
    });
}
