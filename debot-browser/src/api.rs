/*
* Copyright 2018-2021 TON DEV SOLUTIONS LTD.
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
extern crate js_sys;
extern crate web_sys;
use super::config::{resolve_endpoints, Config, UserSettings};
use super::helpers::create_client;
use super::term_signing_box::TerminalSigningBox;
use super::{BrowserHandle, SigningBoxHandle};
use crate::dapp_signing_box::DAppSigningBox;
use crate::{ChainLink, DebotManifest, TerminalBrowser, LOGGER};
use lazy_static::lazy_static;
use log::{info, LevelFilter, SetLoggerError};
use serde::Serialize;
use serde_wasm_bindgen::{from_value, to_value, Serializer};
use std::sync::Arc;
use std::collections::HashMap;
use tokio::sync::{Mutex, RwLock};
use ton_client::crypto::{KeyPair, ParamsOfSign, RegisteredSigningBox, ParamsOfGenerateRandomBytes};
use ton_client::{ClientConfig, ClientContext};
use wasm_bindgen::prelude::*;

lazy_static! {
    static ref BROWSER_TABLE: BrowserTable = BrowserTable::new();
}

struct BrowserTable {
    table: RwLock<HashMap<u64, Arc<Mutex<TerminalBrowser>> > >,
}

impl BrowserTable {
    fn new() -> Self {
        Self { table: RwLock::new(HashMap::new()) }
    }

    async fn insert(&self, browser: TerminalBrowser) -> BrowserHandle {
        let handle = Self::generate_handle();
        self.table.write().await
            .insert(handle.clone(), Arc::new(Mutex::new(browser)));
        handle
    }

    async fn get(&self, handle: &BrowserHandle) -> Option<Arc<Mutex<TerminalBrowser>>> {
        self.table.read().await.get(handle).map(|x| x.clone())
    }

    async fn remove(&self, handle: &BrowserHandle) -> Option<()> {
        self.table.write().await.remove(handle).map(|_| ())
    }

    fn generate_handle() -> BrowserHandle {
        rand::random::<u64>()
    }
}

pub(crate) fn init_log_() -> Result<(), SetLoggerError> {
    log::set_logger(&LOGGER).map(|()| log::set_max_level(LevelFilter::Info))
}

#[wasm_bindgen]
pub fn init_log() -> Result<(), JsValue> {
    init_log_().map_err(|e| to_value(&e.to_string()).unwrap())
}

/// Starts Terminal DeBot Browser with main DeBot.
///
/// Fetches DeBot by address from blockchain and runs it according to pipechain.
#[wasm_bindgen]
pub async fn run_debot_browser(
    url: JsValue,
    wallet: JsValue,
    pubkey: JsValue,
    phrase: JsValue,
    manifest: JsValue,
) -> Result<Option<JsValue>, JsValue> {
    crate::utils::set_panic_hook();
    let url: String = from_value(url)?;
    let wallet: Option<String> = from_value(wallet).ok();
    let pubkey: Option<String> = from_value(pubkey).ok();
    let phrase: Option<String> = from_value(phrase).ok();
    let manifest: String = from_value(manifest).unwrap();
    let mut pipechain: DebotManifest = serde_json::from_str(&manifest).unwrap();
    let addr = pipechain.debot_address.clone();
    let mut debot_config = Config::new();
    debot_config.endpoints = resolve_endpoints(&url);
    debot_config.url = None;

    let ton = create_client(&debot_config)?;
    info!("DEBUG: client created");

    if let Some(phrase) = phrase {
        info!("DEBUG: seed phrase found");
        let input = std::io::BufReader::new(phrase.as_bytes());
        let mut sbox = TerminalSigningBox::new(ton.clone(), vec![], Some(input)).await?;
        let sbox_handle = sbox.leak();
        for cl in pipechain.chain.iter_mut() {
            if let ChainLink::SigningBox { handle } = cl {
                *handle = sbox_handle.0;
            }
        }
    }
    let mut user_settings = UserSettings::default();
    user_settings.wallet = wallet;
    user_settings.pubkey = pubkey;
    let mut browser = TerminalBrowser::new(ton.clone(), user_settings, addr).await?;
    info!("browser created");
    browser.run_manifest(pipechain).await?;
    Ok(to_value(&browser.exit_arg.map(|v| v.to_string())).ok())
}

/// Creates new instance of DeBot Browser and insert it into Global Browser Table.
/// Returns handle as reference for the Browser. This handle can be used later to
/// run Browser or to destroy it.
///
/// endpoint - string with blockchain network url.
/// debot_addr - string with DeBot address.
/// default_wallet - optional user default wallet address. Used by UserInfo interface.
/// default_pubkey - optional user public key. Used by UserInfo interface.
#[wasm_bindgen]
pub async fn create_browser(
    endpoint: String,
    debot_addr: String,
    default_wallet: Option<String>,
    default_pubkey: Option<String>,
) -> Result<BrowserHandle, JsValue> {
    let mut config = Config::new();
    config.endpoints = resolve_endpoints(&endpoint);
    config.url = None;

    let ton = create_client(&config)?;
    info!("client created");

    let mut user_settings = UserSettings::default();
    user_settings.wallet = default_wallet;
    user_settings.pubkey = default_pubkey;

    let browser = TerminalBrowser::new(ton, user_settings, debot_addr).await?;
    info!("browser created");

    Ok(BROWSER_TABLE.insert(browser).await)
}

/// Destroys DeBot browser by its handle.
/// handle - DeBot Browser id in Browser Table.
#[wasm_bindgen]
pub async fn destroy_browser(handle: BrowserHandle) -> Result<(), JsValue> {
    BROWSER_TABLE
        .remove(&handle).await
        .ok_or(format!("invalid handle"))?;
    Ok(())
}

/// Runs previously created DeBot Browser instance.
///
/// handle - number used as reference to DeBot Browser instance created by `create_browser`.
/// manifest - optional object with DeBot manifest.
#[wasm_bindgen]
pub async fn run_browser(handle: BrowserHandle, manifest: JsValue) -> Result<JsValue, JsValue> {
    let manifest: DebotManifest = from_value(manifest).unwrap();
    let browser = BROWSER_TABLE
        .get(&handle).await
        .ok_or(format!("invalid handle"))?;
    let result = browser.lock().await.run_manifest(manifest).await?;
    let serializer = Serializer::new().serialize_maps_as_objects(true);
    let js_result = result.serialize(&serializer).unwrap();
    Ok(js_result)
}

/// Allows to update user settings in DeBot Browser
/// This settings are used by UserInfo interface.
/// handle - DeBot Browser id created by `create_browser`.
/// settings - UserSettings object.
#[wasm_bindgen]
pub async fn update_user_settings(handle: BrowserHandle, settings: JsValue) -> Result<(), JsValue> {
    let browser = BROWSER_TABLE
        .get(&handle).await
        .ok_or(format!("invalid handle"))?;
    let settings: UserSettings =
        from_value(settings).map_err(|e| JsValue::from_str(&e.to_string()))?;
    let lock = browser.lock().await.user_settings.clone();
    let mut user_settings = lock.write().await;
    user_settings.wallet = settings.wallet;
    user_settings.pubkey = settings.pubkey;
    user_settings.signing_box = settings.signing_box;
    Ok(())
}

/// Generates new ed25519 signing keypair
#[wasm_bindgen]
pub fn generate_keypair() -> Result<JsValue, JsValue> {
    let ctx = Arc::new(ClientContext::new(ClientConfig::default()).unwrap());
    let keypair = ton_client::crypto::generate_random_sign_keys(ctx)
        .map_err(|e| JsValue::from_str(&format!("{}", e)))?;
    Ok(to_value(&keypair).unwrap())
}

/// Allows to sign string inside DeBot Browser
#[wasm_bindgen]
pub fn sign(keys: JsValue, unsigned: &[u8]) -> Result<JsValue, JsValue> {
    let keys: KeyPair = from_value(keys).unwrap();
    let ctx = Arc::new(ClientContext::new(ClientConfig::default()).unwrap());
    let result = ton_client::crypto::sign(
        ctx,
        ParamsOfSign {
            keys,
            unsigned: base64::encode(unsigned),
        },
    )
    .map_err(|e| JsValue::from_str(&format!("{}", e)))?;

    to_value(&result).map_err(|e| JsValue::from_str(&e.to_string()))
}

#[wasm_bindgen]
pub async fn register_signing_box(
    handle: BrowserHandle,
    dapp_box: DAppSigningBox,
) -> Result<SigningBoxHandle, JsValue> {
    let browser = BROWSER_TABLE
        .get(&handle).await
        .ok_or(format!("invalid handle"))?;

    let client = browser.lock().await.client.clone();

    let reg_signing_box = ton_client::crypto::register_signing_box(client, dapp_box)
        .await
        .map_err(|e| format!("{}", e))?;
    Ok(reg_signing_box.handle.0)
}

#[wasm_bindgen]
pub async fn close_signing_box(
    handle: BrowserHandle,
    sbox_handle: SigningBoxHandle,
) -> Result<(), JsValue> {
    let browser = BROWSER_TABLE
        .get(&handle).await
        .ok_or(format!("invalid handle"))?;

    let client = browser.lock().await.client.clone();

    ton_client::crypto::remove_signing_box(
        client,
        RegisteredSigningBox {
            handle: ton_client::crypto::SigningBoxHandle(sbox_handle),
        },
    )
    .map_err(|e| format!("{}", e))?;
    Ok(())
}

#[wasm_bindgen]
pub async fn signing_box_public_key(
    handle: BrowserHandle,
    sbox_handle: SigningBoxHandle,
) -> Result<JsValue, JsValue> {
    let browser = BROWSER_TABLE
        .get(&handle).await
        .ok_or(format!("invalid handle"))?;

    let client = browser.lock().await.client.clone();

    let res = ton_client::crypto::signing_box_get_public_key(
        client,
        RegisteredSigningBox {
            handle: ton_client::crypto::SigningBoxHandle(sbox_handle),
        },
    )
    .await
    .map_err(|e| format!("{}", e))?;
    to_value(&res.pubkey).map_err(|e| JsValue::from_str(&e.to_string()))
}

use ton_client::crypto::{ParamsOfChaCha20, ParamsOfHash, ParamsOfScrypt};
#[wasm_bindgen]
pub fn sha256(data: String) -> Result<String, JsValue> {
    let ctx = Arc::new(ClientContext::new(ClientConfig::default()).unwrap());
    ton_client::crypto::sha256(ctx, ParamsOfHash { data })
        .map_err(|e| JsValue::from_str(&format!("{}", e)))
        .map(|v| v.hash)
}

#[wasm_bindgen]
pub fn chacha20(params: JsValue) -> Result<String, JsValue> {
    let params: ParamsOfChaCha20 =
        from_value(params).map_err(|e| JsValue::from_str(&e.to_string()))?;
    let ctx = Arc::new(ClientContext::new(ClientConfig::default()).unwrap());
    ton_client::crypto::chacha20(ctx, params)
        .map_err(|e| JsValue::from_str(&format!("{}", e)))
        .map(|v| v.data)
}

#[wasm_bindgen]
pub fn scrypt(params: JsValue) -> Result<String, JsValue> {
    let params: ParamsOfScrypt =
        from_value(params).map_err(|e| JsValue::from_str(&e.to_string()))?;
    let ctx = Arc::new(ClientContext::new(ClientConfig::default()).unwrap());
    ton_client::crypto::scrypt(ctx, params)
        .map_err(|e| JsValue::from_str(&format!("{}", e)))
        .map(|v| v.key)
}


#[wasm_bindgen]
pub fn generate_random_bytes(length: u32) -> Result<String, JsValue> {
    let ctx = Arc::new(ClientContext::new(ClientConfig::default()).unwrap());
    ton_client::crypto::generate_random_bytes(
        ctx,
        ParamsOfGenerateRandomBytes{ length }
    )
    .map_err(|e| JsValue::from_str(&format!("{}", e)))
    .map(|v| v.bytes)
}