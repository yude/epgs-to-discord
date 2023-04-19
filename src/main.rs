mod client;
mod messages;
mod settings;
mod utils;

use crate::messages::send::send;
use clap::Parser;
use webhook::client::WebhookResult;

/// epgs-to-discord (https://github.com/yude/epgs-to-discord) by yude.
#[derive(Parser)]
#[clap(
    name = env!("CARGO_PKG_NAME"),
    author = env!("CARGO_PKG_AUTHORS"),
    version = env!("CARGO_PKG_VERSION"),
    about = format!(
        "{} (v{}) by {}\nSend EPGStation's notifications to Discord via Webhook.",
        env!("CARGO_PKG_NAME"),
        env!("CARGO_PKG_VERSION"),
        env!("CARGO_PKG_AUTHORS"),
    )
)]
struct Args {
    /// EPGStation's request mode.
    mode: String,
}

#[tokio::main]
async fn main() -> WebhookResult<()> {
    let args = Args::parse();

    send(args.mode).await?;
    Ok(())
}
