//! Module for database interaction and wrapping

pub mod client;
mod models;

pub use {client::Client, mongodb::options::ClientOptions as DatabaseCredentials};
