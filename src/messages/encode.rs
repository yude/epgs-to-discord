use crate::client;
use crate::settings::load::Settings;
use crate::utils::get_executable_path;
extern crate chrono;

use std::env;
use webhook::client::WebhookResult;

pub async fn send_encode(mode: &str) -> WebhookResult<()> {
    let client = client::get_client();

    let settings = Settings::new(&get_executable_path().display().to_string());
    let config = settings;
    let name: &str = &config.unwrap().misc.name;

    let mode_title: &str;
    match mode {
        "finish" => mode_title = ":white_check_mark: エンコード完了",
        &_ => mode_title = "",
    }

    let channel_type = env::var("CHANNELTYPE");
    let channel_type = channel_type.as_ref().map(String::as_str).unwrap_or("");

    let channel_name = env::var("HALF_WIDTH_CHANNELNAME");
    let channel_name = channel_name.as_ref().map(String::as_str).unwrap_or("");

    let program_name = env::var("HALF_WIDTH_NAME");
    let program_name = program_name.as_ref().map(String::as_str).unwrap_or("");

    let encode_mode = env::var("MODE");
    let encode_mode = encode_mode.as_ref().map(String::as_str).unwrap_or("");

    let output_path = env::var("OUTPUTPATH");
    let output_path = output_path.as_ref().map(String::as_str).unwrap_or("");

    client
        .send(|message| {
            message.username(name).embed(|embed| {
                embed
                    .title(mode_title)
                    .field("放送", channel_type, true)
                    .field("局名", channel_name, true)
                    .field("エンコード モード", &encode_mode, true)
                    .field("番組名", program_name, false)
                    .field("保存先", &output_path, false)
            })
        })
        .await?;

    Ok(())
}
