extern crate polychat_core;

use std::env;

use polychat_core::plugin_manager::PluginManager;

fn main() {
    env_logger::init();
    let plugin_dir_path = env::args().nth(1).expect("Usage: ./polychat-basic <absolute path to library folder>");
    let mut manager = PluginManager::from(plugin_dir_path.as_str()).unwrap();
    let services = manager.get_services();
    for service in services {
        let service_name = service.as_str();
        let account = manager.create_account(service_name).unwrap();
        manager.delete_account(service_name, account).unwrap();
    }
}
