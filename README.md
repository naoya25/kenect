# Kenect（ケネクト）

日本の都道府県を隣接関係でつないでいく2人対戦Webゲーム。

ランダムに選ばれた県からスタートし、交互に「隣接する県」を宣言していく。宣言できなくなった、または間違えたプレイヤーの負け。橋・トンネルで結ばれた県（青函トンネルや瀬戸大橋など）も隣接として扱う。

## 技術スタック

| 項目           | 技術                                                  |
| -------------- | ----------------------------------------------------- |
| 言語           | Rust                                                  |
| フロントエンド | [Dioxus](https://dioxuslabs.com/)（WASM）             |
| バックエンド   | [Axum](https://github.com/tokio-rs/axum)              |
| データベース   | SQLite（[sqlx](https://github.com/launchbadge/sqlx)） |

## ディレクトリ構成

```
kenect/
├── Cargo.toml          # ワークスペースルート
├── apps/
│   ├── frontend/       # Dioxus（WASM）
│   └── backend/        # Axum サーバー
├── packages/
│   └── shared/         # 共有型・ゲームロジック
└── assets/             # 日本地図SVGなど
```

## 開発環境のセットアップ

### 必要なもの

- [Rust](https://rustup.rs/)（1.80以上）
- [Dioxus CLI](https://dioxuslabs.com/learn/0.7/getting_started)

```bash
# Dioxus CLI のインストール
cargo install dioxus-cli

# WASMターゲットの追加
rustup target add wasm32-unknown-unknown
```

### 起動方法

**フロントエンド（開発サーバー）**

```bash
cd apps/frontend
dx serve
# http://localhost:8080 で確認
```

**バックエンド**

```bash
cd apps/backend
cargo run
# http://localhost:3000 で起動
```

### ビルド確認

```bash
# ワークスペース全体のチェック
cargo check
```

## ゲームルール

1. ランダムに選ばれた県がスタート地点として表示される
2. プレイヤー1・プレイヤー2が交互に「隣接する県」を宣言する
3. 以下の場合のみ有効な宣言となる
    - 直前の県と隣接している
    - まだ使用されていない
4. 有効な宣言ができなくなったプレイヤーの負け

### 隣接の特別ルール

| 接続          | 路線                   |
| ------------- | ---------------------- |
| 青森 ↔ 北海道 | 青函トンネル           |
| 山口 ↔ 福岡   | 関門橋・関門トンネル   |
| 兵庫 ↔ 徳島   | 明石海峡大橋・大鳴門橋 |
| 岡山 ↔ 香川   | 瀬戸大橋               |
| 広島 ↔ 愛媛   | しまなみ海道           |
