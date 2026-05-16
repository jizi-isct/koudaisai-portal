# 共有ライブラリの設計

すべての共有ライブラリは `package.json` の `exports` で `import`/`default` 条件を `./src/index.ts` に向けており，TypeScript・Vite・Astro がいずれも開発・本番ビルドを通じてソースを直接参照します．`dist/` は使用されません．

`tsconfig.lib.json` は削除せず残しています．`@nx/js/typescript` プラグインがこのファイルを検知して TypeScript のプロジェクト参照チェーンを構築するためです．削除すると `nx typecheck` が壊れます．

すべてのライブラリで `project.json` のビルドタスクを `nx:noop` で無効化しています．型チェックは `nx typecheck` が担い，`build` とは独立して動作します．
