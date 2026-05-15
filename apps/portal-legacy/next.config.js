//@ts-check

// eslint-disable-next-line @typescript-eslint/no-var-requires
const { composePlugins, withNx } = require('@nx/next');


/**
 * @type {import('@nx/next/plugins/with-nx').WithNxOptions}
 **/
const nextConfig = {
  output: 'export',
  images: { unoptimized: true },
  trailingSlash: true,
  transpilePackages:[
    '@koudaisai/shared-api',
    '@koudaisai/shared-auth-members',
    '@koudaisai/shared-ui',
    '@koudaisai/shared-utils',
    '@koudaisai/shared-types',
    '@koudaisai/shared-auth',
  ],
};

const plugins = [
  // Add more Next.js plugins to this list if needed.
  withNx,
];

module.exports = composePlugins(...plugins)(nextConfig);



