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
use crate::SigningBoxHandle;
use lazy_static::lazy_static;
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::sync::Arc;
use tokio::sync::RwLock;

fn default_wc() -> i32 {
    0
}

fn default_retries() -> u8 {
    5
}

fn default_depool_fee() -> f32 {
    0.5
}

fn default_timeout() -> u32 {
    60000
}

fn default_false() -> bool {
    false
}

fn default_true() -> bool {
    true
}

fn default_lifetime() -> u32 {
    60
}

fn default_endpoints() -> Vec<String> {
    return vec![];
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Config {
    pub url: Option<String>,
    #[serde(default = "default_wc")]
    pub wc: i32,
    pub addr: Option<String>,
    pub wallet: Option<String>,
    pub pubkey: Option<String>,
    pub abi_path: Option<String>,
    pub keys_path: Option<String>,
    #[serde(default = "default_retries")]
    pub retries: u8,
    #[serde(default = "default_timeout")]
    pub timeout: u32,
    #[serde(default = "default_false")]
    pub is_json: bool,
    #[serde(default = "default_depool_fee")]
    pub depool_fee: f32,
    #[serde(default = "default_lifetime")]
    pub lifetime: u32,
    #[serde(default = "default_true")]
    pub no_answer: bool,
    #[serde(default = "default_false")]
    pub balance_in_tons: bool,
    #[serde(default = "default_false")]
    pub local_run: bool,
    #[serde(default = "default_false")]
    pub async_call: bool,
    #[serde(default = "default_endpoints")]
    pub endpoints: Vec<String>,
}

#[derive(Deserialize, Clone, Default)]
pub struct UserSettings {
    pub wallet: Option<String>,
    pub pubkey: Option<String>,
    pub signing_box: Option<SigningBoxHandle>,
    /// For compatibility with tonos-cli. Remove when possible.
    pub keys_path: Option<String>,
}

pub type SharedUserSettings = Arc<RwLock<UserSettings>>;
pub fn make_shared_settings(settings: UserSettings) -> SharedUserSettings {
    Arc::new(RwLock::new(settings))
}

#[derive(Serialize, Deserialize, Clone)]
pub struct FullConfig {
    config: Config,
    endpoints_map: BTreeMap<String, Vec<String>>,
}

impl Config {
    pub fn new() -> Self {
        let url = None;
        let endpoints = vec![];
        Config {
            url,
            wc: default_wc(),
            addr: None,
            wallet: None,
            pubkey: None,
            abi_path: None,
            keys_path: None,
            retries: default_retries(),
            timeout: default_timeout(),
            is_json: default_false(),
            depool_fee: default_depool_fee(),
            lifetime: default_lifetime(),
            no_answer: default_true(),
            balance_in_tons: default_false(),
            local_run: default_false(),
            async_call: default_false(),
            endpoints,
        }
    }

    #[cfg(not(target_arch = "wasm32"))]
    pub fn from_file(path: &str) -> Option<Self> {
        let conf_str = std::fs::read_to_string(path).ok()?;
        let conf: serde_json::error::Result<FullConfig> = serde_json::from_str(&conf_str);
        conf.map(|c| c.config)
            .or_else(|_| serde_json::from_str(&conf_str))
            .ok()
    }

    #[cfg(not(target_arch = "wasm32"))]
    pub fn to_file(path: &str, conf: &Config) -> Result<(), String> {
        let mut fconf = FullConfig::from_file(path);
        fconf.config = conf.to_owned();
        FullConfig::to_file(path, &fconf)
    }
}

lazy_static! {
    static ref MAIN_ENDPOINTS: Vec<String> = vec![
        "https://main2.ton.dev".to_string(),
        "https://main3.ton.dev".to_string(),
        "https://main4.ton.dev".to_string(),
    ];
    static ref NET_ENDPOINTS: Vec<String> = vec![
        "https://net1.ton.dev".to_string(),
        "https://net5.ton.dev".to_string(),
    ];
    static ref SE_ENDPOINTS: Vec<String> = vec![
        "http://0.0.0.0/".to_string(),
        "http://127.0.0.1/".to_string(),
        "http://localhost/".to_string(),
    ];
}

pub fn resolve_net_name(url: &str) -> Option<String> {
    let url_regex = Regex::new(r"^\s*(?:https?://)?(?P<net>\w+\.ton\.dev)\s*")
        .expect("Regex compilation error");
    if let Some(captures) = url_regex.captures(url) {
        if let Some(net) = captures.name("net") {
            let network = net.as_str();
            if FullConfig::default_map().contains_key(network) {
                return Some(network.to_owned());
            }
        }
    }
    if url.contains("127.0.0.1") || url.contains("0.0.0.0") || url.contains("localhost") {
        return Some("http://127.0.0.1/".to_string());
    }
    None
}

pub(crate) fn resolve_endpoints(url: &str) -> Vec<String> {
    match resolve_net_name(url) {
        Some(network) => FullConfig::default_map()[&network].clone(),
        None => vec![url.to_string()],
    }
}

impl FullConfig {
    pub fn new() -> Self {
        FullConfig {
            config: Config::new(),
            endpoints_map: FullConfig::default_map(),
        }
    }
    pub fn default_map() -> BTreeMap<String, Vec<String>> {
        [
            ("main.ton.dev".to_owned(), MAIN_ENDPOINTS.to_owned()),
            ("net.ton.dev".to_owned(), NET_ENDPOINTS.to_owned()),
            ("http://127.0.0.1/".to_owned(), SE_ENDPOINTS.to_owned()),
        ]
        .iter()
        .cloned()
        .collect()
    }

    #[allow(dead_code)]
    pub fn get_map(path: &str) -> BTreeMap<String, Vec<String>> {
        FullConfig::from_file(path).endpoints_map
    }
    pub fn from_file(path: &str) -> FullConfig {
        let conf_str = std::fs::read_to_string(path).ok().unwrap_or_default();
        serde_json::from_str(&conf_str)
            .ok()
            .unwrap_or(FullConfig::new())
    }

    pub fn to_file(path: &str, fconf: &FullConfig) -> Result<(), String> {
        let conf_str = serde_json::to_string_pretty(fconf)
            .map_err(|_| "failed to serialize config object".to_string())?;
        std::fs::write(path, conf_str)
            .map_err(|e| format!("failed to write config file: {}", e))?;
        Ok(())
    }
}
#[cfg(test)]
mod tests {
    use super::resolve_net_name;

    #[test]
    fn test_endpoints_resolver() {
        assert_eq!(resolve_net_name(""), None);
        assert_eq!(resolve_net_name("http://os.ton.dev"), None);
        assert_eq!(resolve_net_name("https://rustnet.ton.dev"), None);
        assert_eq!(resolve_net_name("rustnet.ton.com"), None);
        assert_eq!(resolve_net_name("https://example.com"), None);
        assert_eq!(
            resolve_net_name("http://localhost"),
            Some("http://127.0.0.1/".to_owned())
        );
        assert_eq!(
            resolve_net_name("https://localhost"),
            Some("http://127.0.0.1/".to_owned())
        );
        assert_eq!(
            resolve_net_name("localhost"),
            Some("http://127.0.0.1/".to_owned())
        );
        assert_eq!(
            resolve_net_name("http://127.0.0.1"),
            Some("http://127.0.0.1/".to_owned())
        );
        assert_eq!(
            resolve_net_name("https://127.0.0.1"),
            Some("http://127.0.0.1/".to_owned())
        );
        assert_eq!(resolve_net_name("https://127.0.0.2"), None);
        assert_eq!(resolve_net_name("https://127.1.0.1"), None);
        assert_eq!(resolve_net_name("https://0.0.0.1"), None);
        assert_eq!(resolve_net_name("https://1.0.0.0"), None);

        assert_eq!(
            resolve_net_name("https://main.ton.dev"),
            Some("main.ton.dev".to_owned())
        );
        assert_eq!(
            resolve_net_name("http://main.ton.dev"),
            Some("main.ton.dev".to_owned())
        );
        assert_eq!(
            resolve_net_name("  http://main.ton.dev  "),
            Some("main.ton.dev".to_owned())
        );
        assert_eq!(
            resolve_net_name("  https://main.ton.dev  "),
            Some("main.ton.dev".to_owned())
        );
        assert_eq!(
            resolve_net_name("main.ton.dev"),
            Some("main.ton.dev".to_owned())
        );
        assert_eq!(resolve_net_name("main.ton.com"), None);

        assert_eq!(
            resolve_net_name("https://net.ton.dev"),
            Some("net.ton.dev".to_owned())
        );
        assert_eq!(
            resolve_net_name("http://net.ton.dev"),
            Some("net.ton.dev".to_owned())
        );
        assert_eq!(
            resolve_net_name("  http://net.ton.dev  "),
            Some("net.ton.dev".to_owned())
        );
        assert_eq!(
            resolve_net_name("  https://net.ton.dev  "),
            Some("net.ton.dev".to_owned())
        );
        assert_eq!(
            resolve_net_name("net.ton.dev"),
            Some("net.ton.dev".to_owned())
        );
        assert_eq!(resolve_net_name("net.ton.com"), None);
    }
}
