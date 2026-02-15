# keybd-sleep

Windows GUI アプリ。
PCをスリープ状態にする。
キーボードイベントを送信し、Windows の「クイックリンクメニュー」の「シャットダウンまたはサインアウト」メニューからスリープ動作を実行する。
キーボードイベント送信後、即終了する。常駐等は行わない。

## コマンドライン

```
Usage: KeybdSleep.exe [/N] [/Q] [milliseconds]
```

## オプション

* /N = 実際のスリープ操作を行わない
* /Q = 確認ダイアログを出さない
* 数値 = キー送信間の待ち時間（ミリ秒）

## Licence

This software is released under the MIT License, see LICENSE.
