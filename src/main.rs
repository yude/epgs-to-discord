use webhook::client::{WebhookClient, WebhookResult};

mod settings;

use settings::Settings;

#[tokio::main]
async fn main() -> WebhookResult<()> {
    let settings = Settings::new();

    let config = settings;
    // let url: &str = ;
    let url: &str = &config.unwrap().credentials.webhook_url;
    let client: WebhookClient = WebhookClient::new(url);
    client
        .send(|message| {
            message.username("EPGStation").embed(|embed| {
                embed
                    .title("Webhook")
                    .description("Hello, World!")
                    .field("name", "value", false)
            })
        })
        .await?;

    Ok(())
}
