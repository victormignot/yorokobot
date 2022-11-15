//! Discord bot providing a subscription system to topics.
//! Powered by [`Serenity`]
//!
//! [`Serenity`]: https://github.com/serenity-rs/serenity

#![deny(missing_docs)]
#![deny(warnings)]

mod client;
mod database;
mod environment;
pub mod errors;

pub use crate::{
    client::{Client, ClientCredentials},
    database::DatabaseCredentials,
};
