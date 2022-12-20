extern crate polychat_core;
extern crate polychat_plugin;

use std::env;
use log::error;

use polychat_core::main::Main;
use crate::polychat_plugin::plugin::CoreInterface;

fn main() {
    env_logger::init();
    let dir_path = env::args().nth(1).expect("Usage: ./polychat-basic <absolute path to library>");
    let mut main = Main::new();
    
    match main.init(&dir_path) {
        Err(e) => error!("Error loading plugin directory, {}", e),
        Ok(_) => main.test("We good!".to_string())
    };
}
