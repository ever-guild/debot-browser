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
use crate::config::Config;
use std::sync::Arc;
use ton_client::abi::{Abi, AbiConfig, AbiContract};
use ton_client::crypto::{CryptoConfig, MnemonicDictionary};
use ton_client::{ClientConfig, ClientContext};

pub const HD_PATH: &str = "m/44'/396'/0'/0/0";
pub const WORD_COUNT: u8 = 12;

pub fn read_keys(filename: &str) -> Result<ton_client::crypto::KeyPair, String> {
    let keys_str = std::fs::read_to_string(filename)
        .map_err(|e| format!("failed to read the keypair file: {}", e.to_string()))?;
    let keys: ton_client::crypto::KeyPair =
        serde_json::from_str(&keys_str).map_err(|e| format!("failed to load keypair: {}", e))?;
    Ok(keys)
}

pub fn load_ton_address(addr: &str, wc: i32) -> Result<String, String> {
    let addr = if addr.find(':').is_none() {
        format!("{}:{}", wc, addr)
    } else {
        addr.to_owned()
    };
    Ok(addr)
}

pub type TonClient = Arc<ClientContext>;

pub fn create_client_local() -> Result<TonClient, String> {
    let cli = ClientContext::new(ClientConfig::default())
        .map_err(|e| format!("failed to create tonclient: {}", e))?;
    Ok(Arc::new(cli))
}

pub fn create_client(conf: &Config) -> Result<TonClient, String> {
    let cli_conf = ClientConfig {
        abi: AbiConfig {
            workchain: conf.wc,
            message_expiration_timeout: conf.timeout,
            message_expiration_timeout_grow_factor: 1.3,
        },
        crypto: CryptoConfig {
            mnemonic_dictionary: MnemonicDictionary::English,
            mnemonic_word_count: WORD_COUNT,
            hdkey_derivation_path: HD_PATH.to_string(),
        },
        network: ton_client::net::NetworkConfig {
            server_address: conf.url.clone(),
            endpoints: if conf.endpoints.is_empty() {
                None
            } else {
                Some(conf.endpoints.to_owned())
            },
            network_retries_count: 3,
            message_retries_count: conf.retries as i8,
            message_processing_timeout: 30000,
            wait_for_timeout: 30000,
            out_of_sync_threshold: (conf.timeout / 2),
            max_reconnect_timeout: 1000,
            ..Default::default()
        },
        ..Default::default()
    };
    let cli =
        ClientContext::new(cli_conf).map_err(|e| format!("failed to create tonclient: {}", e))?;
    Ok(Arc::new(cli))
}

pub fn load_abi(abi: &str) -> Result<Abi, String> {
    Ok(Abi::Contract(
        serde_json::from_str::<AbiContract>(abi)
            .map_err(|e| format!("ABI is not a valid json: {}", e))?,
    ))
}
