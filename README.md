# ScreenQRReader
パソコンの画面上に表示されているQRコードやバーコードを読み取る事ができるソフトです。
コードの内容がURLだった場合はブラウザで開くことが出来ます。
また、QRコードを生成する機能もあります。メニューから「QRコードメーカー」を選択して使用してください。

## インストール
[リリースページ](https://github.com/KNSN92/ScreenQRReader/releases/latest)から対応したバージョンをダウンロードしてインストールしてください。

## 使用技術
- [rust](https://github.com/rust-lang/rust) (使用言語)
- [tauri](https://github.com/tauri-apps/tauri) (使用フレームワーク)
- [rxing](https://github.com/rxing-core/rxing) (QRコードの読み取りに使用)
- [react](https://react.dev/) (フロントエンドのUI構築に使用)
- [tailwindcss](https://tailwindcss.com/) (スタイリングに使用)
- [qrcode-styling](https://github.com/kozakdenys/qr-code-styling) (スタイリングされたQRコードの生成に使用)

## 今後の予定
- Windowsへの対応
- Linuxへの対応
- ショートカットキーのキーバインド設定の追加
- 複数のQRコードを読み取った際にどのQRコードを開くか選択出来る機能
- ~~QRコードを生成できる機能~~ ✅
- ダイアログなどに専用の見た目を使用

## ビルド
Tauriの標準的な手順に従ってください。~~不親切？~~

## ライセンス
LGPL2.1

QRコードは株式会社デンソーウェーブの登録商標です。
