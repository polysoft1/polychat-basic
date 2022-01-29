extern crate polychat_core;

use std::env;

use polychat_core::plugin::Plugin;

fn main() {
    env_logger::init();
    let plugin_path = env::args().nth(1).expect("Usage: ./polychat-basic <absolute path to library>");
    let loaded_plugin = &mut Plugin::new(&plugin_path).expect("Could not load plugin");

    let acc = loaded_plugin.create_account();
    loaded_plugin.print(acc);
    loaded_plugin.delete_account(acc);
}
