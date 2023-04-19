use crate::messages::encode;
use crate::messages::error;
use crate::messages::record;
use crate::messages::reserve;

use webhook::client::WebhookResult;

pub async fn send(mode: String) -> WebhookResult<()> {
    let m: &str = &mode[..];
    match m {
        "reserve" | "update" | "deleted" | "prestart" | "prepfailed" => {
            reserve::send_reserve(m).await?
        }
        "start" | "end" | "recfailed" => record::send_record(m).await?,
        "finish" => encode::send_encode(m).await?,
        &_ => error::send_error().await?,
    }
    Ok(())
}
