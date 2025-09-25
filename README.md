# ScreenQRReader
パソコンの画面上に表示されているQRコードやバーコードを読み取る事ができるソフトです。
コードの内容がURLだった場合はブラウザで開くことが出来ます。

## 使用技術
- [rust](https://github.com/rust-lang/rust) (使用言語)
- [tauri](https://github.com/tauri-apps/tauri) (使用フレームワーク)
- [zbar](https://github.com/mchehab/zbar) (QRコードの読み取りに使用)

## 今後の予定
- グローバルショートカットキーを変更できる設定
- Windowsへの対応
- linuxへの対応
- 複数のQRコードを読み取った際にどのQRコードを開くか選択出来る機能
- QRコードを生成できる機能
- ダイアログなどに専用の見た目を使用
