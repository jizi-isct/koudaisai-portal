npx openapi-typescript api_v2/openapi.yml --output ../apps/web/src/lib/api_v2.d.ts
npx openapi-typescript auth_v1/openapi.yml --output ../apps/web/src/lib/auth_v1.d.ts
npx openapi-typescript https://raw.githubusercontent.com/jizi-isct/koudaisai-plans-info-api/refs/heads/main/docs/openapi.yml --output ../apps/web/src/lib/plans_info_api_v1.d.ts