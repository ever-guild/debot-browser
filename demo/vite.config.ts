import { defineConfig } from 'vite'
import wasm from 'vite-plugin-wasm'
import topLevelAwait from 'vite-plugin-top-level-await'
const isProduction = process.argv.includes('production')

export default defineConfig({
  base: isProduction ? '/debot-browser/' : '/',
  plugins: [
    wasm(),
    topLevelAwait()
  ],
})
