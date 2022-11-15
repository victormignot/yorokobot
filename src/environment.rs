//! Utility function to deal wit environment variable

use log::error;
use std::env;

/// Get the environment variiable var_name or panic
pub fn get_env_variable(var_name: &String) -> String {
    match env::var(var_name) {
        Ok(v) => v,
        Err(_) => {
            error!(target: "bot_warn_errors", "Failed to fetch the {} environment variable.", var_name);
            panic!("Exiting...");
        }
    }
}
