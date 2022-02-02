extern crate polychat_core;

use std::env;

use polychat_core::plugin_manager::PluginManager;

fn main() {
    env_logger::init();
    let plugin_dir_path = env::args().nth(1).expect("Usage: ./polychat-basic <absolute path to library>");
    PluginManager::from(plugin_dir_path.as_str());
}
