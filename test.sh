#!/bin/sh

RUST_BACKTRACE=1 CHANNELTYPE=GR HALF_WIDTH_CHANNELNAME=チャンネル名 HALF_WIDTH_NAME=番組タイトル HALF_WIDTH_DESCRIPTION=ここに番組概要が入ります。 HALF_WIDTH_EXTENDED=ここに番組詳細が入ります。 STARTAT=1681879153 ENDAT=1681879153 ERROR_CNT=2 DROP_CNT=3 SCRAMBLING_CNT=4 MODE=foo OUTPUTPATH=/foo/bar/test.mp4 cargo run -- finish