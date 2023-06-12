import { fileURLToPath, URL } from 'node:url'

import { defineConfig } from 'vite'
import vue from '@vitejs/plugin-vue'
import vueJsx from '@vitejs/plugin-vue-jsx'
import { ViteRsw } from 'vite-plugin-rsw'

// https://vitejs.dev/config/
export default defineConfig({
  plugins: [vue(), vueJsx(), ViteRsw()],
  resolve: {
    alias: {
      '@': fileURLToPath(new URL('./src', import.meta.url))
    }
  },
  server: {
	  host: true
  }
})
