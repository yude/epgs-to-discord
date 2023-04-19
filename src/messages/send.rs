use crate::messages::reserve;
use webhook::client::WebhookResult;

pub async fn send(mode: String) -> WebhookResult<()> {
    if mode == "reserve" {
        reserve::send_reserve().await?;
    } else {
        reserve::send_reserve().await?;
    }

    Ok(())
}
