//! Common Yorokobot errors

use mongodb::error::Error as MongoError;
use serenity::prelude::SerenityError;

/// The kind of errors that can be returned by Client::new
#[derive(Debug)]
pub enum ClientsError {
    /// Serenity error while building client
    Discord(SerenityError),

    ///Mongo error while parsing options
    Database(MongoError),
}
