//@ts-check

// eslint-disable-next-line @typescript-eslint/no-var-requires
const { composePlugins, withNx } = require('@nx/next');


/**
 * @type {import('@nx/next/plugins/with-nx').WithNxOptions}
 **/
const nextConfig = {
  output: 'export',  // 静的サイトとしてビルド
  images: {
    unoptimized: true  // 画像最適化を無効化（SSGに必要）
  },
  trailingSlash: true,  // URLの末尾にスラッシュを追加
};

const plugins = [
  // Add more Next.js plugins to this list if needed.
  withNx,
];

module.exports = composePlugins(...plugins)(nextConfig);

