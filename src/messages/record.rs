use crate::client;
use crate::settings::load::Settings;
use crate::utils::get_executable_path;
extern crate chrono;

use chrono::{Local, TimeZone};
use std::env;
use webhook::client::WebhookResult;

pub async fn send_record(mode: &str) -> WebhookResult<()> {
    let client = client::get_client();

    let settings = Settings::new(&get_executable_path().display().to_string());
    let config = settings;
    let name: &str = &config.unwrap().misc.name;

    let mode_title: &str;
    match mode {
        "start" => mode_title = ":record_button: 録画開始",
        "end" => mode_title = ":stop_button: 録画終了",
        "recfailed" => mode_title = ":warning: 録画失敗",
        &_ => mode_title = "",
    }

    let channel_type = env::var("CHANNELTYPE");
    let channel_type = channel_type.as_ref().map(String::as_str).unwrap_or("");

    let channel_name = env::var("HALF_WIDTH_CHANNELNAME");
    let channel_name = channel_name.as_ref().map(String::as_str).unwrap_or("");

    let program_name = env::var("HALF_WIDTH_NAME");
    let program_name = program_name.as_ref().map(String::as_str).unwrap_or("");

    let program_desc = env::var("HALF_WIDTH_DESCRIPTION");
    let program_desc = program_desc.as_ref().map(String::as_str).unwrap_or("");

    let program_extended = env::var("HALF_WIDTH_EXTENDED");
    let program_extended = program_extended.as_ref().map(String::as_str).unwrap_or("");

    let program_start_at_unixtime = env::var("STARTAT");
    let program_start_at_unixtime = program_start_at_unixtime
        .as_ref()
        .map(String::as_str)
        .unwrap_or("");
    let program_start_at = Local
        .timestamp_millis_opt(program_start_at_unixtime.parse::<i64>().unwrap() * 1000)
        .unwrap()
        .format("%Y/%m/%d %H:%M:%S %Z")
        .to_string();

    let program_end_at_unixtime = env::var("ENDAT");
    let program_end_at_unixtime = program_end_at_unixtime
        .as_ref()
        .map(String::as_str)
        .unwrap_or("");
    let program_end_at = Local
        .timestamp_millis_opt(program_end_at_unixtime.parse::<i64>().unwrap() * 1000)
        .unwrap()
        .format("%Y/%m/%d %H:%M:%S %Z")
        .to_string();

    let error_cnt = env::var("ERROR_CNT");
    let error_cnt = error_cnt.as_ref().map(String::as_str).unwrap_or("N/A");

    let drop_cnt = env::var("DROP_CNT");
    let drop_cnt = drop_cnt.as_ref().map(String::as_str).unwrap_or("N/A");

    let scrambling_cnt = env::var("SCRAMBLING_CNT");
    let scrambling_cnt = scrambling_cnt.as_ref().map(String::as_str).unwrap_or("N/A");

    client
        .send(|message| {
            message.username(name).embed(|embed| {
                embed
                    .title(mode_title)
                    .field("放送", channel_type, true)
                    .field("局名", channel_name, true)
                    .field("番組名", program_name, false)
                    .field("番組概要", program_desc, false)
                    .field("番組詳細", program_extended, false)
                    .field("開始日時", &program_start_at, true)
                    .field("終了日時", &program_end_at, true)
                    .field("", "", true)
                    .field("エラー", &error_cnt, true)
                    .field("ドロップ", &drop_cnt, true)
                    .field("スクランブル", &scrambling_cnt, true)
            })
        })
        .await?;

    Ok(())
}
