use crate::messages::error;
use crate::messages::reserve;
use webhook::client::WebhookResult;

pub async fn send(mode: String) -> WebhookResult<()> {
    let m: &str = &mode[..];
    match m {
        "reserve" => reserve::send_reserve("reserve").await?,
        &_ => error::send_error().await?,
    }
    Ok(())
}
