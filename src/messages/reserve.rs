use crate::client;
use webhook::client::WebhookResult;

pub async fn send_reserve() -> WebhookResult<()> {
    let client = client::get_client();

    client
        .send(|message| {
            message.username("EPGStation").embed(|embed| {
                embed.title("Webhook").description("Hello, World!").field(
                    "Executable path",
                    "aa",
                    false,
                )
            })
        })
        .await?;

    Ok(())
}
