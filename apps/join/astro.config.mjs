import { defineConfig, envField } from 'astro/config';
import cloudflare from '@astrojs/cloudflare';
import react from '@astrojs/react';

export default defineConfig({
  integrations: [react()],
  output: 'server',
  site: 'https://join.koudaisai.jp',
  env: {
    schema: {
      API_URL: envField.string({ context: 'client', access: 'public' }),
      GA_MEASUREMENT_ID: envField.string({
        context: 'client',
        access: 'public',
        optional: true,
      }),
    },
  },
  adapter: cloudflare({
    prerenderEnvironment: 'node',
  }),
});
