//! Bot data models

#![allow(dead_code)]

use serde::{Deserialize, Serialize};

/// All the models within Mongo COllections
pub enum CollectionModels {
    /// Discord Guild
    Guild(Guild),

    /// Yorokobot tags
    Tag(Tag),
}

/// Settings for a server
#[derive(Debug, Serialize, Deserialize)]
pub struct GuildSettings {
    admin_only_can_tag: bool,
    server_ban_list: Vec<String>,
}

/// Server infos
#[derive(Debug, Serialize, Deserialize)]
pub struct Guild {
    discord_guild_id: String,
    bot_settings: GuildSettings,
}

/// Tags
#[derive(Debug, Serialize, Deserialize)]
pub struct Tag {
    name: String,
    guild: String,
    is_nsfw: bool,
    subscribers: Vec<String>,
}
