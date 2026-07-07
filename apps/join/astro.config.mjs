import { defineConfig, envField } from 'astro/config';
import react from '@astrojs/react';

export default defineConfig({
  integrations: [react()],
  output: 'static',
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
});
