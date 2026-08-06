#!/usr/bin/env bash
# OpenAPI 仕様を backend(utoipa)から再生成し、TypeScript クライアント型を生成する。
# api_v3 / auth_v2 は backend のコードが単一の真実なので毎回 dump し直す。
# events26(企画情報API)は外部 API なので spec を URL から直接引く。書き込みは
# backend の /api/v3/events26 経由だが、読み取り(企画一覧・場所一覧)は events26 が
# 直接公開しており backend に中継が無いため、フロントも events26 の型を要る。
set -euo pipefail
cd "$(dirname "$0")"

# sqlx の query! マクロをオフライン(コミット済み .sqlx キャッシュ)で検査させる。
# apps/backend/.cargo/config.toml にも同設定があるが、cargo の config 探索は
# manifest ではなくカレントディレクトリ起点なので、ここ(docs/api)から
# --manifest-path で起動すると読まれない。CI(クリーンビルド)で live DB へ
# 接続しようとして失敗するため、明示的に設定する。
export SQLX_OFFLINE=true

BACKEND=../../apps/backend/Cargo.toml
cargo run --quiet --manifest-path "$BACKEND" -- --dump-openapi=api_v3 > api_v3/openapi.json
cargo run --quiet --manifest-path "$BACKEND" -- --dump-openapi=auth_v2 > auth_v2/openapi.json

npx -y openapi-typescript api_v3/openapi.json --output ../../libs/shared-types/src/api_v3.d.ts
npx -y openapi-typescript auth_v2/openapi.json --output ../../libs/shared-types/src/auth_v2.d.ts
npx -y openapi-typescript https://events26.koudaisai.jp/openapi.json --output ../../libs/shared-types/src/events26.d.ts

# 生成物を repo の prettier 設定で整形する(コミット済み .d.ts の規約に合わせる)。
npx prettier --write \
  ../../libs/shared-types/src/api_v3.d.ts \
  ../../libs/shared-types/src/auth_v2.d.ts \
  ../../libs/shared-types/src/events26.d.ts
