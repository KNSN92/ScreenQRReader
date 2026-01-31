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

## MacOSでインストールするときの追加手順
Apple Developer Programに加入する資金力が存在しないため、署名をしていません。
そのためこのままだとアプリを開いても"壊れているため開けません"のような表示が出て開けません。
開けるようにするには以下のコマンドを入力してください。
<br>`xattr -cr "ScreenQRReaderまでのパス"`<br>
もしくは自前でビルドするのも手ですが。


## ライセンス
LGPL2.1 (zbar-rust からの継承)

QRコードは株式会社デンソーウェーブの登録商標です。