# 関数型プログラミング学習

関数型プログラミングの学びをまとめたレポ。

教科書は『なっとく関数型プログラミング』。本のコードは Java / Scala だが、練習問題は Rust で書く。

## 構成

- `practice/`：練習問題のコード（Cargo プロジェクト）。章ごとに `src/ch02.rs` のようにファイルを分ける
- `note/`：章ごとの学習メモ（`ch02.md` など）

## テストの実行

```bash
cd practice
cargo test          # 全章をテスト
cargo test ch02     # 2章だけテスト
```

## 新しい章を始めるとき

1. `practice/src/ch03.rs` を作る
2. `practice/src/lib.rs` に `pub mod ch03;` を追加する
3. `note/ch03.md` にメモを書く
