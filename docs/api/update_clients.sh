npx -y openapi-typescript api_v2/openapi.yml --output ../../libs/shared-types/src/api_v2.d.ts
npx -y openapi-typescript auth_v1/openapi.yml --output ../../libs/shared-types/src/auth_v1.d.ts
npx -y openapi-typescript https://raw.githubusercontent.com/jizi-isct/koudaisai-plans-info-api/refs/heads/main/docs/openapi.yml --output ../../libs/shared-types/src/plans_info_api_v1.d.ts