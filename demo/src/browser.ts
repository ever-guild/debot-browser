// import * as browser from '@ever-guild/debot-browser'
// import * as browser from 'debot-browser'
import * as browser from '@debug/debot-browser'
import defaultManifest from './manifest/default.json'
import sendManifest from './manifest/send.json'

let keypair = {
  public: '9f7fd3df9d72b133fe155c087928c4f9da423076cc20c9f5386614b462e49811',
  secret: '607a90aedb5df02a0f712572f0b5aa5d9342e5f3f2c0794df43f4a2a9688aef3',
}
// browser.sign(keypair, unsigned)
const projectId = 'bf520b125fe24b96b4545f4358d2edba'
const network = 'devnet'
const endpoint = `https://${ network }.evercloud.dev/${ projectId }/graphql`
const debotTest = '0:2d2696edfe3d7c0d74e8900b2a43ac362de5a45db6fe6147177e2fcd2abfd3e2'
const debotSend = '0:af5acb55481ebd9c923c11462ea6ab068508a0e05aacd8a9c33bb5b0cabcc33c'
const wallet = '0:2f9f742cd3ed63c39a31c077d5faada4e52ea365a4b4a9e1d6709e6cb0e9d927'
const pubkey = `0x${ keypair.public }`
const callCount = 3

export async function setupBrowser(element: HTMLElement) {
  const logger = (messge: string): void => {
    element.textContent += `${ messge }\n`
  }
  defaultManifest.debotAddress = debotTest
  defaultManifest.initArgs.arg6 = pubkey
  sendManifest.debotAddress = debotSend
  sendManifest.initArgs.dest = debotSend
  logger(`NETWORK=${ network }`)
  logger(`DEBOT=${ debotTest }`)
  let result
  // FIXME Uncaught (in promise) RuntimeError: unreachable
  browser.init_log()
  logger('Test 1. run_debot_browser')
  console.time('Test 1. Run browser (single call)')
  result = await browser.run_debot_browser(endpoint, wallet, pubkey, null, defaultManifest)
  browser.init_log()
  console.timeEnd('Test 1')
  logger('Result:')
  logger(result)
  logger('Test 1. Completed')

  logger(`Test 2. Create, run, destroy browser (${ callCount } calls)`)
  const handle = await browser.create_browser(endpoint, debotTest, wallet, pubkey)
  console.time('Test 2')
  for (let i = 0; i < callCount; i++) {
    result = await browser.run_browser(handle, defaultManifest)
  }
  console.timeEnd('Test 2')
  console.log('Result:', result)
  logger(JSON.stringify(result))
  await browser.destroy_browser(handle)
  logger('Test 2. Completed')

  logger('Test 3. Create 3 browsers in parallel')
  console.time('Test 3')
  const handle1 = browser.create_browser(endpoint, debotTest, wallet, pubkey)
  const handle2 = browser.create_browser(endpoint, debotTest, wallet, pubkey)
  const handle3 = browser.create_browser(endpoint, debotTest, wallet, pubkey)
  const handles = await Promise.all([handle1, handle2, handle3])
  console.log(`handle1 = ${ handles[0] } handle2 = ${ handles[1] } handle3 = ${ handles[2] }`)
  console.timeEnd('Test 3')
  logger('Test 3. Completed')

  // const handle = await browser.create_browser(endpoint, debotTest, wallet, pubkey);
  // const result = await browser.run_browser(handle, defaultManifest);
  // console.log(result)
  // let counter = 0
  // const setCounter = (count: number) => {
  //   counter = count
  //   element.innerHTML = `count is ${counter}`
  // }
  // element.addEventListener('click', () => setCounter(counter + 1))
  // setCounter(0)
}
