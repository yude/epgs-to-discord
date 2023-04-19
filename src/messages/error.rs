use crate::client;
use crate::settings::load::Settings;
use crate::utils::get_executable_path;
use webhook::client::WebhookResult;

pub async fn send_error() -> WebhookResult<()> {
    let client = client::get_client();

    let settings = Settings::new(&get_executable_path().display().to_string());
    let config = settings;
    let name: &str = &config.unwrap().misc.name;

    client
        .send(|message| {
            message.username(name).embed(|embed| {
                embed
                    .title("Error")
                    .description("You have an error on the EPGStation's configuration.")
            })
        })
        .await?;

    Ok(())
}
