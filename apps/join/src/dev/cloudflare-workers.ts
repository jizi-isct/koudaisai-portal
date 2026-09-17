// Local development reads public values from `astro:env/client` instead.
// This shim keeps Cloudflare's runtime-only module out of the Node.js dev server.
export const env = {} as Env;
