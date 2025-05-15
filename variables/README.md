## **研究テーマ**
**Rust vs JavaScript(+WASM)**

## **目的**
Webブラウザ上でRust（WASM）とJavaScriptを使った同じ処理（速度、メモリ管理と安全性、書きやすさ、デバッグなど）を比較し、どちらが速く、効率的で、使いやすいかを明らかにする。

## **背景**
Rustはメモリ管理が厳密で安全、速度に優れる。
JavaScriptはブラウザの標準言語であり、開発が容易で、リッチなフロントエンドでよく使われている。
WebAssembly（WASM）は、JavaScriptの処理を補完するために、ブラウザで高性能なコードを実行するための技術。

## Rust環境構築

VisualStudio installerを開き

- インストール済みの Visual Studio の右側に「**変更**」ボタンがあるのでクリック
- 「**ワークロード**」タブが表示されたら…
    
    👉「**C++によるデスクトップ開発**」にチェックを入れる ✅
    
    - （言語パックで「英語」も必要って出てたら、それもチェックでOK）
- 右下の「**変更を適用**」を押して、必要なコンポーネントを追加インストール


Rustの公式サイトからRustをインストール
インストールしたファイルを開くと
```bash
1) Proceed with installation (default)
2) Customize installation
3) Cancel installation
```
または
```bash
Rust is installed now. Great!
```
とでるのでそのままEnter
コマンドプロンプトでバージョン確認

```bash
rustc --version←Rustのバージョン確認
```

```bash
cargo --version←Cargoのバージョン確認
```

## VSCodeを開き拡張機能を追加する
自分が追加した機能↓
- rust←メイン
- Rust Doc Viewer←`cargo doc`で生成されたドキュメントを VSCode 上で表示できるようになる
- Rust Test Explorer←テストをサイドバーから一覧　　表示して実行できるようになる
- rust-analyzer←公式が開発してるLSP
- Even Better TOML←Cargo.tomlで補完が効くようになる
- CodeLLDB←デバッグするのに必要

これで環境構築は終了！
