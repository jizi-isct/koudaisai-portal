//@ts-check

 
const { composePlugins, withNx } = require('@nx/next');


/**
 * @type {import('@nx/next/plugins/with-nx').WithNxOptions}
 **/
const nextConfig = {
  output: 'export',
  images: { unoptimized: true },
  trailingSlash: true,
  transpilePackages: [
    '@koudaisai/shared-api',
    '@koudaisai/shared-auth',
    '@koudaisai/shared-auth-admin',
    '@koudaisai/shared-ui',
  ],
};

const plugins = [
  // Add more Next.js plugins to this list if needed.
  withNx,
];

module.exports = composePlugins(...plugins)(nextConfig);

