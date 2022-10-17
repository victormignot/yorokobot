use serenity::{prelude::GatewayIntents, Client as DiscordClient};

/// Yorokobot's client.
/// Used for connecting to the Discord API and your MongoDB database
///
/// # Example
/// ```rust,no_run
/// use yorokobot::client::{Client, ClientCredentials};
///
/// let discord_token = "Your discord token";
///
/// let credentials = ClientCredentials {
///     discord_token,
/// };
///
/// let client = Client::new(credentials);
///
/// client.connect().await
/// ```
///
pub struct Client {
    discord_client: DiscordClient,
}

/// Yorokobot connection credentials
pub struct ClientCredentials<'a> {
    /// Token for Discord API
    pub discord_token: &'a String,
}

impl<'a> Client {
    /// Create a Yorokobot client
    pub async fn new(credentials: ClientCredentials<'a>) -> Client {
        let discord_client =
            DiscordClient::builder(credentials.discord_token, GatewayIntents::empty())
                .await
                .expect("Could not create Discord Client");

        Client { discord_client }
    }

    /// Start connection to Discord API.
    /// Wrap [`serenity::client::Client`] start method.
    pub async fn connect_discord(&mut self) {
        if let Err(error) = self.discord_client.start().await {
            println!("Could not connect to Discord: {:?}", error);
        }
    }

    /// Connect client to the Mongo database then to the Discord API.
    pub async fn connect(&mut self) {
        self.connect_discord().await;
    }
}
