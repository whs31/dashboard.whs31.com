import { defineConfig } from 'vite'
import { svelte } from '@sveltejs/vite-plugin-svelte'
import tailwindcss from '@tailwindcss/vite'
import path from "node:path";

export default defineConfig({
  plugins: [svelte({ compilerOptions: { runes: true } }), tailwindcss()],
  resolve: {
    alias: {
      $lib: path.resolve('./src/lib')
    }
  },

  server: {
    proxy: {
      '/api': {
        target: 'http://localhost:3000',
        changeOrigin: true
      }
    }
  }
})
