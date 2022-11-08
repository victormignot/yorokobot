//! Common Yorokobot errors

pub use mongodb::error::Error as DatabaseError;
pub use serenity::prelude::SerenityError as DiscordError;

/// The kind of errors that can be returned by Client::new
#[derive(Debug)]
pub enum ClientError {
    /// Serenity error while building client
    Discord(DiscordError),

    ///Mongo error while parsing options
    Database(DatabaseError),
}
