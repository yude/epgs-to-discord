use std::env;
use std::path::PathBuf;
use webhook::client::WebhookClient;

use crate::settings::load::Settings;

pub fn get_client() -> WebhookClient {
    let mut current_path = PathBuf::new();
    match env::current_exe() {
        Ok(exe_path) => {
            current_path = exe_path;
            current_path.pop();
        }
        Err(e) => println!("Failed to get current executable path: {e}"),
    };
    let settings = Settings::new(current_path.display().to_string());

    let config = settings;
    let url: &str = &config.unwrap().credentials.webhook_url;
    let client: WebhookClient = WebhookClient::new(url);

    return client;
}

// fn get_current_working_dir() -> String {
//     let res = env::current_dir();
//     match res {
//         Ok(path) => path.into_os_string().into_string().unwrap(),
//         Err(_) => "FAILED".to_string(),
//     }
// }
