# EPGS-to-Discord
Send EPGStation notifications to Discord via Webhook  
Currently this software does NOT support i18n (only Japanese).

# Setup
* Add the following into your configuration of EPGStation.
    * You can ignore unnecessary lines that you don't need.
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

## License
MIT