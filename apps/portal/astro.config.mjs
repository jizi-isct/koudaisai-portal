import { defineConfig, envField } from 'astro/config';
import react from '@astrojs/react';

export default defineConfig({
  integrations: [react()],
  output: 'static',
  env: {
    schema: {
      API_URL: envField.string({ context: 'client', access: 'public' }),
      AUTH_URL: envField.string({ context: 'client', access: 'public' }),
      PLANS_INFO_API_URL: envField.string({
        context: 'client',
        access: 'public',
      }),
    },
  },
});
