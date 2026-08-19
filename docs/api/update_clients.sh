#!/usr/bin/env bash
# OpenAPI 仕様から TypeScript クライアント型を生成する。
# api_v3 / auth_v2 の openapi.json は backend の dump-openapi ターゲットが生成する
# (nx の dependsOn 経由。backend のソースが変わらない限りキャッシュが効く)ため、
# ここでは cargo を起動しない。
# events26(企画情報API)は外部 API なので spec を URL から直接引く。書き込みは
# backend の /api/v3/events26 経由だが、読み取り(企画一覧・場所一覧)は events26 が
# 直接公開しており backend に中継が無いため、フロントも events26 の型を要る。
#
# --check: 生成後にコミット済みの生成物との差分を表示し、差分があれば失敗する
#          (CI の仕様ドリフト検知用。生成そのものは通常どおり実行される)。
set -euo pipefail
cd "$(dirname "$0")"

CHECK=false
if [[ "${1:-}" == "--check" ]]; then
  CHECK=true
fi

npx -y openapi-typescript api_v3/openapi.json --output ../../libs/shared-types/src/api_v3.d.ts
npx -y openapi-typescript auth_v2/openapi.json --output ../../libs/shared-types/src/auth_v2.d.ts
npx -y openapi-typescript https://events26.koudaisai.jp/openapi.json --output ../../libs/shared-types/src/events26.d.ts

# 生成物を repo の prettier 設定で整形する(コミット済み .d.ts の規約に合わせる)。
npx prettier --write \
  ../../libs/shared-types/src/api_v3.d.ts \
  ../../libs/shared-types/src/auth_v2.d.ts \
  ../../libs/shared-types/src/events26.d.ts

if [[ "$CHECK" == false ]]; then
  exit 0
fi

# ドリフト検知。dump-openapi が書く openapi.json も対象に含める。
GENERATED=(
  docs/api/api_v3/openapi.json
  docs/api/auth_v2/openapi.json
  libs/shared-types/src/api_v3.d.ts
  libs/shared-types/src/auth_v2.d.ts
  libs/shared-types/src/events26.d.ts
)
cd ../..
# HEAD と比較する(index 経由の比較だと stage 済みの差分を見逃すため)。
if git --no-pager diff HEAD --exit-code -- "${GENERATED[@]}"; then
  echo "OpenAPI 仕様と生成物は同期しています。"
  exit 0
fi

cat >&2 <<'EOS'

コミット済みの OpenAPI 仕様/クライアント型が最新ではありません(上の差分を参照)。
ローカルで次を実行し、生成物をコミットしてください:

  npx nx run docs-api:update-clients

events26.d.ts のみ差分が出る場合、外部 API(events26)側の変更です。
EOS
exit 1
