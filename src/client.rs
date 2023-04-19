use webhook::client::WebhookClient;

use crate::settings::load::Settings;
use crate::utils::get_executable_path;

pub fn get_client() -> WebhookClient {
    let settings = Settings::new(&get_executable_path().display().to_string());
    let config = settings;
    let url: &str = &config.unwrap().credentials.webhook_url;

    let client: WebhookClient = WebhookClient::new(url);

    return client;
}
