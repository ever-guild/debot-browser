# Changelog

All notable changes to this project will be documented in this file. 

## 0.6.0 (2023-05-??)

* updated dependencies and ever-sdk to 1.42.1
* rename package to `@ever-guild/debot-browser`

## 0.5.5 (2022-04-05)

## Fixes

* Switched to ton_client 1.32.0

## 0.5.4 (2022-03-??)

## Fixes

* Switched to ton_client 1.31.0

## 0.5.1 (2022-02-28)

## Fixes

* Fixed memory leak in browser (caused by `wee_alloc` crate)

## 0.5.0 (2021-12-29)

### Features

* Exported crypto functions: sha256, scrypt, chacha20

## 0.4.5 (2021-12-28)

## Fixes

* Fixed parallel browser runs

## 0.4.4 (2021-12-27)

### Chore

* Switched to ton_client 1.28.0

## 0.4.3 (2021-12-24)

### Fixes

* Supported unknown endpoints

## 0.4.2 (2021-12-20)

### Fixes

* Fixed bug in sign wrapper for signing box.

## 0.4.1 (2021-12-20)

### Fixes

* Fixed README

## 0.4.0 (2021-12-20)

### Features

* Added support for signing boxes: new functions `register_signing_box`, `close_signing_box`, `update_user_settings`.

## 0.3.0 (2021-12-12)

### Features

* Added new API functions: `create_browser`, `run_browser` and `destroy_browser`.
* Added support for DeBots with ABI 2.2.
* Updated test script in `index.js`.

## 0.2.0 (2021-09-00)

### Features

* Added support for `Query` interface in DEngine.

## 0.1.0 (2021-09-00)

### Features

  - MVP of wasm debot browser.
  - add `run_debot_browser` API function.
  - add test server.
