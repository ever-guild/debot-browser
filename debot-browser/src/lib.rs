/*
* Copyright 2018-2020 TON DEV SOLUTIONS LTD.
*
* Licensed under the SOFTWARE EVALUATION License (the "License"); you may not use
* this file except in compliance with the License.
*
* Unless required by applicable law or agreed to in writing, software
* distributed under the License is distributed on an "AS IS" BASIS,
* WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
* See the License for the specific TON DEV software governing permissions and
* limitations under the License.
*/
mod api;
mod callbacks;
mod config;
mod convert;
mod crypto;
mod dapp_signing_box;
mod helpers;
mod interfaces;
mod pipechain;
mod processor;
pub mod term_browser;
mod term_encryption_box;
mod term_signing_box;
mod utils;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

pub use api::{
    close_signing_box, create_browser, destroy_browser, generate_keypair, init_log,
    register_signing_box, run_browser, run_debot_browser, sign, signing_box_public_key,
    update_user_settings, scrypt, sha256, chacha20, generate_random_bytes
};
use callbacks::Callbacks;
pub use interfaces::dinterface::SupportedInterfaces;
use log::{Level, Metadata, Record};
use pipechain::{ApproveKind, ChainLink, DebotManifest};
use processor::{ChainProcessor, ProcessorError};
use term_browser::{action_input, input, terminal_input, TerminalBrowser};

type BrowserHandle = u64;
type SigningBoxHandle = u32;

struct DeBotBrowserLogger;
static LOGGER: DeBotBrowserLogger = DeBotBrowserLogger;

impl log::Log for DeBotBrowserLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= Level::Info
    }

    #[cfg(target_arch = "wasm32")]
    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            web_sys::console::log_1(&format!("{} - {}", record.level(), record.args()).into());
        }
    }

    #[cfg(not(target_arch = "wasm32"))]
    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            println!("{} - {}", record.level(), record.args());
        }
    }

    fn flush(&self) {}
}
