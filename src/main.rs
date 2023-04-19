mod client;
mod messages;
mod settings;

use crate::messages::send::send;
use webhook::client::WebhookResult;

#[tokio::main]
async fn main() -> WebhookResult<()> {
    send(String::from("reserve")).await?;
    Ok(())
}
