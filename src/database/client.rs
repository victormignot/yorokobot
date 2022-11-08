use mongodb::Client as MongoClient;

use crate::errors::ClientError;
use crate::DatabaseCredentials;

/// Database client
pub struct Client<'a> {
    mongo_client: Option<MongoClient>,
    // database: Option<Database>,
    credentials: &'a DatabaseCredentials,
}

impl<'a> Client<'a> {
    /// Create a new database client
    pub fn new(credentials: &'a DatabaseCredentials) -> Client {
        return Client {
            credentials,
            mongo_client: None,
            // database: None,
        };
    }

    /// Connect the client
    pub async fn connect(&mut self) -> Result<(), ClientError> {
        self.mongo_client = match MongoClient::with_options(self.credentials.clone()) {
            Ok(c) => Some(c),
            Err(e) => return Err(ClientError::Database(e)),
        };

        if let None = self.mongo_client.as_ref().unwrap().default_database() {
            // TODO:
            // Implement an Environment Variable catcher to wrap std::env::var()
            // As we often call it and always have to use a match control flow

            // TODO:
            // Complete error kind to be more specific.
            // Ex: DatabaseConnection

            todo!();
        }

        Ok(())
    }
}
