//! Module containing the Yorokobot client and used structs

use crate::errors::ClientsError;

use mongodb::{options::ClientOptions as MongoClientOptions, Client as MongoClient};
use serenity::{prelude::GatewayIntents, Client as DiscordClient};

/// Yorokobot's client.
/// Used for connecting to the Discord API and your MongoDB database
///
/// # Example
/// ```rust,no_run
/// # async fn run() {
/// use yorokobot::client::{Client, ClientCredentials};
///
/// let token = String::from("Your discord token");
///
/// let credentials = ClientCredentials {
///     discord_token: &token,
/// };
///
/// let mut client = Client::new(credentials).await;
///
/// client.connect().await;
///
/// # }
/// ```
pub struct Client {
    /// The Serenity Discord Client
    discord_client: DiscordClient,

    /// The MongoDB Client
    mongodb_client: Option<MongoClient>,

    /// MongoDB Client Options
    mongodb_options: MongoClientOptions,
}

/// Yorokobot connection credentials
pub struct ClientCredentials<'a> {
    /// Token for Discord API
    pub discord_token: &'a String,

    /// MongoDB connection string.
    pub mongo_uri: &'a String,
}

impl<'a> Client {
    /// Create a Yorokobot client
    pub async fn new(credentials: ClientCredentials<'a>) -> Result<Self, ClientsError> {
        let discord_client = match DiscordClient::builder(
            credentials.discord_token,
            GatewayIntents::empty(),
        )
        .await
        {
            Ok(c) => c,
            Err(e) => return Err(ClientsError::Discord(e)),
        };

        let mongodb_options = match MongoClientOptions::parse(credentials.mongo_uri).await {
            Ok(o) => o,
            Err(e) => return Err(ClientsError::Database(e)),
        };

        Ok(Client {
            discord_client,
            mongodb_options,
            mongodb_client: None,
        })
    }

    /// Start connection to Discord API.
    /// Wrap [`serenity::client::Client`] start method.
    pub async fn connect_discord(&mut self) -> Result<(), ClientsError> {
        match self.discord_client.start().await {
            Ok(_) => Ok(()),
            Err(e) => return Err(ClientsError::Discord(e)),
        }
    }

    /// Connect to the Mongo Database
    pub fn connect_mongodb(&mut self) -> Result<(), ClientsError> {
        self.mongodb_client = match MongoClient::with_options(self.mongodb_options.clone()) {
            Ok(c) => Some(c),
            Err(e) => return Err(ClientsError::Database(e)),
        };

        Ok(())
    }

    /// Connect client to the Mongo database then to the Discord API.
    pub async fn connect(&mut self) -> Result<(), ClientsError> {
        self.connect_mongodb()?;
        self.connect_discord().await?;

        Ok(())
    }
}
