import './style.css'
import { setupBrowser } from './browser.ts'

async function main() {
  document.querySelector<HTMLDivElement>('#app')!.innerHTML = `
  <div>
    <a href="https://www.npmjs.com/package/@ever-guild/debot-browser" target="_blank">
      <img src="https://raw.githubusercontent.com/ever-guild/debot/main/asset/debot.svg" class="logo" alt="debot logo" />
    </a>
    <h1>Bebot browser</h1>
    <div class="card">
      <pre>
        <code id="log"></code>
      </pre>
    </div>
  </div>
`
  await setupBrowser(document.querySelector<HTMLElement>('#log')!)
}

main().catch(console.error)
