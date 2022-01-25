extern crate polychat_core;

use std::env;

use polychat_core::plugin::Plugin;

fn main() {
    let plugin_path = env::args().nth(1).expect("Usage: ./polychat-basic <absolute path to library>");
    let loaded_plugin = &mut Plugin::new("example", &plugin_path).expect("Could not load plugin");

    let acc = loaded_plugin.create_account().expect("Could not create account for plugin");
    loaded_plugin.print(acc).expect("Could not print account");
}
