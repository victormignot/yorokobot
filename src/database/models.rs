//! Bot data models

#![allow(dead_code)]

use serde::{Deserialize, Serialize};

pub const COLLECTIONS_NAMES: [&str; 2] = ["guilds", "tags"];

pub trait YorokobotModel {
    fn get_collection_name() -> String;
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

impl YorokobotModel for Guild {
    fn get_collection_name() -> String {
        return "guilds".to_string();
    }
}
impl YorokobotModel for Tag {
    fn get_collection_name() -> String {
        return "traits".to_string();
    }
}
