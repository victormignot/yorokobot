use log::error;
use std::env;

use yorokobot::{
    client::{Client, ClientCredentials},
    errors::ClientsError,
};

#[tokio::main]
async fn main() -> std::process::ExitCode {
    // Start the logger
    log4rs::init_file("log4rs_config.yaml", Default::default()).unwrap();

    let discord_token = match env::var("DISCORD_TOKEN") {
        Ok(t) => t,
        Err(_) => {
            error!(target: "bot_warn_errors", "Could not find DISCORD_TOKEN environment variable.");
            return std::process::ExitCode::FAILURE;
        }
    };

    let mongodb_uri = match env::var("MONGODB_URI") {
        Ok(u) => u,
        Err(_) => {
            error!(target: "bot_warn_errors", "Could not find MONGODB_URI environment variable.");
            return std::process::ExitCode::FAILURE;
        }
    };

    let credentials = ClientCredentials {
        discord_token: &discord_token,
        mongo_uri: &mongodb_uri,
    };

    let mut client = match Client::new(credentials).await {
        Ok(c) => c,
        Err(_) => {
            error!(target: "bot_warn_errors", "Could not instantiate the bot client.");
            return std::process::ExitCode::FAILURE;
        }
    };

    if let Err(error) = client.connect().await {
        match error {
            ClientsError::Database(e) => {
                error!(target: "bot_warn_errors", "Could not connect to database: {:?}", e)
            }
            ClientsError::Discord(e) => {
                error!(target: "bot_warn_errors", "Could not connect to Discord: {:?}", e)
            }
        };

        return std::process::ExitCode::FAILURE;
    }

    std::process::ExitCode::SUCCESS
}
