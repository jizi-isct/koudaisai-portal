# AGENTS.md

このファイルは、このリポジトリで作業するエージェント向けのガイダンスを提供します。

## プロジェクト概要

工大祭（東京科学大学大岡山キャンパスの文化祭）のポータルサイトです。団体は申請の提出、各種リソースの閲覧、JIZI（工大祭実行委員会）との連絡を行います。フロントエンド複数アプリと Rust/Axum バックエンドで構成された Nx モノレポです。

## よく使うコマンド

```bash
# 依存関係をインストール（install ではなく ci を使用）
npm ci

# 開発起動（フロントエンド各アプリ + バックエンドをホットリロードで起動）
npx nx dev

# Docker サービス起動（PostgreSQL, Keycloak）
npx nx docker-up backend

# ビルド
npx nx build portal   # 参加団体向けポータル（フロントエンド）
npx nx build @koudaisai-portal/admin  # 管理画面（フロントエンド）
npx nx build join     # Join フロントエンド
npx nx build backend  # バックエンド（Rust release）

# テスト
npx nx test backend   # cargo test

# Lint
npx nx lint portal    # ESLint
npx nx lint @koudaisai-portal/admin
npx nx lint join
npx nx lint backend

# DB マイグレーション（apps/backend/ 内で実行）
sea-orm-cli migrate up
sea-orm-cli generate entity  # DB スキーマから SeaORM エンティティを生成
```

## アーキテクチャ

### モノレポ構成

- `apps/portal/` - 参加団体向けポータル（フロントエンド）
- `apps/admin/` - 管理画面（フロントエンド）
- `apps/join/` - Join フロントエンド
- `apps/backend/` - Rust/Axum バックエンド（SeaORM, PostgreSQL）
- `docs/` - OpenAPI 仕様とドキュメント

### バックエンドレイヤー（`apps/backend/src/`）

1. `routes/` - HTTP エンドポイント
2. `middlewares.rs` - 認証、ロギング
3. `entities/` - ビジネスロジック
4. `sea_orm_entities/` - DB モデル（自動生成）
5. `service/` - 外部連携（Discord, S3）
6. `util/` - JWT、OIDC、ハッシュ化

### 認証

- **JIZI（管理者）**: Keycloak OIDC
- **Group（参加団体）**: 初回ログイン時に有効化するカスタム JWT

### 外部サービス

- PostgreSQL（データ）、S3/Wasabi（ファイル）、Keycloak（認証）
- Discord（通知）、SendGrid/SES（メール）
- 外部 API: api2025.jizi.jp（企画情報同期）

## ドメイン用語

- **JIZI**: 工大祭実行委員会（管理者）
- **Group**: 参加団体または学内取材団体
- **参加団体種別**: Booth（模擬店 M-xxx）、Stage（ステージ S-xxx）、General（一般 I-xxx）、Labo（研究室 L-xxx）
- **Press**: 学内取材団体（P-xxx）

## 開発環境セットアップ

このリポジトリでは Nix flakes を使って開発に必要なツールを揃えます（主なツール: Rust、Node.js、cargo-watch、git）。

### 事前に必要なもの

- Nix
- direnv
- Docker daemon を起動できる環境（Docker Desktop など）

> Docker と Docker Compose は開発用 DB・Keycloak の起動に使用します。`npx nx docker-up backend` の実行前に Docker daemon を起動してください。

### セットアップ手順

1. `direnv allow` を実行して Nix の開発環境を有効化
2. リポジトリルートで `npm ci` を実行
3. JWT キーを生成: `cd apps/backend/debug && ./init-keys.sh`
4. コンフィグを配置
   - `npx nx dev backend` または `npx nx dev` で起動する場合、`apps/backend/debug/default-config.toml` は自動で読み込まれます。
   - 手動で backend を起動する場合は、`cd apps/backend/debug && cp default-config.toml <配置先>` を実行します。
   - `<配置先>`:
     - macOS: `~/Library/Application Support/rs.koudaisai-portal/`
     - Linux: `~/.config/koudaisai-portal/`
     - Windows: `C:\Users\<ユーザー名>\AppData\Roaming\rs.koudaisai-portal\`
   - または環境変数 `KOUDAISAI_PORTAL_CONFIG_PATH` でコンフィグファイルのパスを指定できます。
5. 開発環境用 DB・Keycloak を起動: `npx nx docker-up backend`
6. 開発環境を起動: `npx nx dev`

利用 URL:

- portal: `http://portal.koudaisai.localhost`
- admin: `http://admin.koudaisai.localhost`
- join: `http://join.koudaisai.localhost`
- backend API: `http://api.koudaisai.localhost`

> Caddy が 80 番ポートを利用できない環境では、管理者権限での実行が必要になる場合があります。

## ブランチ命名規則

```
<prefix>/#<issue>-<short-title>
```

プレフィックス: feature, fix, hotfix, refactor, chore, test
