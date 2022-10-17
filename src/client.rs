use serenity::{prelude::GatewayIntents, Client as DiscordClient};

pub struct Client {
    discord_client: DiscordClient,
}

pub struct ClientOptions<'a> {
    pub discord_token: &'a String,
}

impl<'a> Client {
    pub async fn new(options: ClientOptions<'a>) -> Client {
        let discord_client = DiscordClient::builder(options.discord_token, GatewayIntents::empty())
            .await
            .expect("Could not create Discord Client");

        Client { discord_client }
    }

    pub async fn connect_discord(&mut self) -> () {
        if let Err(error) = self.discord_client.start().await {
            println!("Could not connect to Discord: {:?}", error);
        }
    }
}
