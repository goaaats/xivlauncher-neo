import {defineConfig} from 'vite'
import vue from '@vitejs/plugin-vue'
import {quasar} from '@quasar/vite-plugin'
import eslintPlugin from 'vite-plugin-eslint'
import * as path from 'path'

// https://vitejs.dev/config/
export default defineConfig({
  build: {
    target: 'esnext'
  },
  logLevel: 'info',
  plugins: [
    vue({}),
    quasar({
      autoImportComponentCase: 'kebab',
      sassVariables: '@/css/quasar.sass',
    }),
    eslintPlugin({}),
  ],
  resolve: {
    alias: {
      '@': path.resolve(__dirname, './src'),
    }
  },
  server: {
    host: '127.0.0.1',
    port: 8080,
    strictPort: true,
  }
})
