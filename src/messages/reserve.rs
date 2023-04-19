use crate::client;
use crate::settings::load::Settings;
use crate::utils::get_executable_path;
use webhook::client::WebhookResult;

pub async fn send_reserve(mode: &str) -> WebhookResult<()> {
    let client = client::get_client();

    let settings = Settings::new(&get_executable_path().display().to_string());
    let config = settings;
    let name: &str = &config.unwrap().misc.name;

    let mode_title: &str;
    match mode {
        "reserve" => mode_title = "🆕 予約追加",
        "update" => mode_title = "🔃 予約更新",
        "deleted" => mode_title = "🗑 予約削除",
        "prestart" => mode_title = "🆚 予約準備",
        "prepfailed" => mode_title = "❌ 予約準備失敗",
        &_ => mode_title = "",
    }

    client
        .send(|message| {
            message.username(name).embed(|embed| {
                embed.title(mode_title).description("Hello, World!").field(
                    "Executable path",
                    "aa",
                    false,
                )
            })
        })
        .await?;

    Ok(())
}
