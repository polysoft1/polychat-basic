extern crate polychat_core;
extern crate polychat_plugin;

use std::env;
use log::{error, info};

use polychat_core::core::Core;

fn main() {
    env_logger::init();
    let dir_path = env::args().nth(1).expect("Usage: ./polychat-basic <absolute path to library>");
    let mut main = Core::new();
    
    match main.init(&dir_path) {
        Err(e) => error!("Error loading plugin directory, {}", e),
        Ok(_) => {
            let plugins = main.get_plugin_names();
            info!("Found {} plugins", plugins.len());
            for plugin in plugins {
                info!("\tFound {} plugin for {} protocol", plugin.clone(), main.get_plugin_by_name(plugin).unwrap().get_protocol_name());
            }
        }
    };
}
