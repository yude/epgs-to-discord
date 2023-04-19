# EPGS-to-Discord
⏺ EPGStation の通知を Discord に送信します

## 設定
* 以下を EPGStation の設定ファイル (`config/config.yml`) に追記します。
    * 通知を希望しないものについては、その行を無視できます。
    ```
    isEnabledDropCheck: true
    reserveNewAddtionCommand: './epgs-to-discord reserve'
    reserveUpdateCommand: './epgs-to-discord update'
    reservedeletedCommand: './epgs-to-discord deleted'
    recordingPreStartCommand: ./epgs-to-discord prestart'
    recordingPrepRecFailedCommand: './epgs-to-discord prepfailed'
    recordingStartCommand: './epgs-to-discord start'
    recordingFinishCommand: './epgs-to-discord end'
    recordingFailedCommand: './epgs-to-discord recfailed'
    encodingFinishCommand: './epgs-to-discord finish'
    ```
* `$HOME/.config/epgs-to-discord/config.toml` を作成します。内容は以下のようにし、適宜必要な値を入力します。
    ```toml
    [misc]
    # ボットの名前
    name = "EPGStation"

    [credentials]
    # Discord 上チャンネルに対応する Webhook の URL
    webhook_url = ""
    ```

## ライセンス
MIT