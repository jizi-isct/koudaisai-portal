#!/usr/bin/env bash
# OpenAPI 仕様を backend(utoipa)から再生成し、人間向け HTML リファレンスを生成する。
# backend のコードが単一の真実なので、まず spec を dump してから redocly でビルドする。
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

npx -y @redocly/cli build-docs ./api_v3/openapi.json --output api_v3.html
npx -y @redocly/cli build-docs ./auth_v2/openapi.json --output auth_v2.html
