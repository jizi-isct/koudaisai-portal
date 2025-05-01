import {defineConfig} from 'vite';
import react from '@vitejs/plugin-react';
import dts from 'vite-plugin-dts';
import * as path from 'path';
import preserveDirectives from "rollup-plugin-preserve-directives";

export default defineConfig(() => ({
  root: __dirname,
  cacheDir: '../../node_modules/.vite/libs/ui-generic',
  plugins: [react(), dts({
    entryRoot: 'src',
    tsconfigPath: path.join(__dirname, 'tsconfig.lib.json')
  }), dts({entryRoot: 'src', tsconfigPath: path.join(__dirname, 'tsconfig.lib.json')}),],
  // Uncomment this if you are using workers.
  // worker: {
  //  plugins: [ nxViteTsPaths() ],
  // },
  // Configuration for building your library.
  // See: https://vitejs.dev/guide/build.html#library-mode
  build: {
    'emptyOutDir': true,
    'transformMixedEsModules': true,
    'entry': 'src/index.ts',
    'name': '@koudaisai-portal/ui-show_document',
    'fileName': 'index',
    'formats': ['es' as const],
    'external': ['react', 'react-dom', 'react/jsx-runtime'],
    'output': {
      preserveModules: true
    },
    'preserveModules': true,
    'plugins': [preserveDirectives()],
    'lib': {
      'entry': "src/index.ts",
      'name': "@koudaisai-portal/ui-show_document",
      'fileName': "index",
      'formats': ['es' as const],
    },
    'rollupOptions': {"external": ["'react'", "'react-dom'", "'react/jsx-runtime'"]},
    'outDir': "./dist",
    'reportCompressedSize': true,
    'commonjsOptions': {"transformMixedEsModules": true},
  },
}));
