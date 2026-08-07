import { defineConfig, envField } from 'astro/config';
import react from '@astrojs/react';

export default defineConfig({
  integrations: [react()],
  output: 'static',
  env: {
    schema: {
      API_URL: envField.string({ context: 'client', access: 'public' }),
      AUTH_URL: envField.string({ context: 'client', access: 'public' }),
      // 企画情報API(events26)の公開読み取り用ベース URL。
      // api_v3 は admin 向けの書き込みしか中継しないため、参照は直接叩く。
      EVENTS26_API_URL: envField.string({
        context: 'client',
        access: 'public',
      }),
    },
  },
});
