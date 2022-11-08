//! Module containing the Yorokobot client and used structs

use crate::{database::Client as DatabaseClient, errors::ClientError, DatabaseCredentials};

use serenity::{prelude::GatewayIntents, Client as DiscordClient};

/// Yorokobot's client.
/// Used for connecting to the Discord API and your MongoDB database
///
/// # Example
/// ```rust,no_run
/// # async fn run() {
/// use yorokobot::client::{Client, ClientCredentials};
///
/// let discord_token = String::from("Your discord token");
/// let mongo_uri = String::from("Your Mongo URI");
///
/// let credentials = ClientCredentials {
///     discord_token: &discord_token,
///     mongo_uri: &mongo_uri,
/// };
///
/// let mut client = Client::new(credentials).await.expect("Error creating client");
///
/// client.connect().await;
///
/// # }
/// ```
pub struct Client<'a> {
    /// The Serenity Discord Client
    discord_client: DiscordClient,

    /// The database client
    database_client: DatabaseClient<'a>,
}

/// Yorokobot connection credentials
pub struct ClientCredentials<'a> {
    /// Token for Discord API
    pub discord_token: &'a String,

    /// MongoDB connection string.
    pub db_credentials: &'a DatabaseCredentials,
}

impl<'a> Client<'a> {
    /// Create a Yorokobot client
    pub async fn new(credentials: ClientCredentials<'a>) -> Result<Client, ClientError> {
        let discord_client = match DiscordClient::builder(
            credentials.discord_token,
            GatewayIntents::empty(),
        )
        .await
        {
            Ok(c) => c,
            Err(e) => return Err(ClientError::Discord(e)),
        };

        let database_client = DatabaseClient::new(credentials.db_credentials);

        Ok(Client {
            discord_client,
            database_client,
        })
    }

    /// Start connection to Discord API.
    /// Wrap [`serenity::client::Client`] start method.
    pub async fn connect_discord(&mut self) -> Result<(), ClientError> {
        match self.discord_client.start().await {
            Ok(_) => Ok(()),
            Err(e) => Err(ClientError::Discord(e)),
        }
    }

    /// Connect client to the Mongo database then to the Discord API.
    pub async fn connect(&mut self) -> Result<(), ClientError> {
        self.database_client.connect().await?;
        self.connect_discord().await?;

        Ok(())
    }
}
