import { defineConfig, envField } from 'astro/config';
import cloudflare from '@astrojs/cloudflare';
import react from '@astrojs/react';
import { fileURLToPath } from 'node:url';

// Avoid the Cloudflare Vite runtime bug that crashes `astro dev` with
// `Missing field moduleType`; production builds still use the real adapter.
const isDevCommand = process.argv.includes('dev');

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
  adapter: isDevCommand
    ? undefined
    : cloudflare({
        prerenderEnvironment: 'workerd',
      }),
  vite: isDevCommand
    ? {
        resolve: {
          alias: {
            'cloudflare:workers': fileURLToPath(
              new URL('./src/dev/cloudflare-workers.ts', import.meta.url),
            ),
          },
        },
      }
    : undefined,
});
