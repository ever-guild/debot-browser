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
use super::config::{make_shared_settings, UserSettings, SharedUserSettings};
use super::helpers::{load_abi, load_ton_address, TonClient};
use super::{Callbacks, ChainProcessor, DebotManifest, SupportedInterfaces};
use log::info;
use serde_json::json;
use std::collections::{HashMap, VecDeque};
use std::io::{self, BufRead, Write};
use std::sync::Arc;
use ton_client::abi::{
    decode_message, encode_internal_message, Abi, CallSet, ParamsOfDecodeMessage,
    ParamsOfEncodeInternalMessage,
};
use ton_client::boc::{parse_message, ParamsOfParse};
use ton_client::debot::{DEngine, DebotInfo, DebotInterfaceExecutor, DEBOT_WC};

const BROWSER_ID: &'static str = "0000000000000000000000000000000000000000000000000000000000000000";
/// Stores Debot info needed for DBrowser.
struct DebotEntry {
    abi: Abi,
    dengine: DEngine,
    callbacks: Arc<Callbacks>,
    info: DebotInfo,
}

/// Top level object. Created only once.
pub struct TerminalBrowser {
    /// Instance of SDK client. 
    pub client: TonClient,
    /// User Information used by UserInfo interface
    pub user_settings: SharedUserSettings,
    /// Address of starting DeBot.
    main_debot_addr: String,
    /// common message queue for both inteface calls and invoke calls (from different debots).
    msg_queue: VecDeque<String>,
    /// Map of instantiated Debots. [addr] -> entry.
    /// New debots are created by invoke requests.
    bots: HashMap<String, DebotEntry>,
    /// Browser callbacks for DEngine.
    callbacks: Arc<Callbacks>,
    /// Set of intrefaces implemented by current DBrowser.
    interfaces: SupportedInterfaces,
    /// Input chain Processor for Debot Manifest 
    processor: Arc<tokio::sync::RwLock<ChainProcessor>>,
    /// Indicates if Browser will interact with the user or not.
    interactive: bool,
    /// Browser exit argument. Initialized only if DeBot sends message to the DeBot Browser address.
    pub exit_arg: Option<serde_json::Value>,
}

impl TerminalBrowser {
    pub async fn new(
        client: TonClient,
        user_settings: UserSettings,
        addr: String,
    ) -> Result<Self, String> {
        let processor = Arc::new(tokio::sync::RwLock::new(ChainProcessor::new()));
        
        let callbacks = Arc::new(
            Callbacks::new(
                client.clone(),
                processor.clone(),
            )
        );

        let user_settings = make_shared_settings(user_settings);

        let interfaces = SupportedInterfaces::new(
            client.clone(), 
            user_settings.clone(),
            processor.clone(),
        );

        // TODO remove clone
        let main_debot_addr = addr.clone();
        
        let mut browser = Self {
            client,
            user_settings,
            main_debot_addr,
            interfaces,
            callbacks,
            processor,
            msg_queue: Default::default(),
            bots: Default::default(),
            interactive: false,
            exit_arg: None,
        };

        let _ = browser.fetch_debot(&addr, false, true).await?;

        Ok(browser)
    }

    async fn fetch_debot(
        &mut self,
        addr: &str,
        call_start: bool,
        autorun: bool,
    ) -> Result<String, String> {
        let debot_addr = load_ton_address(addr, 0)?;
        let callbacks_ref = Arc::clone(&self.callbacks);
        let mut dengine = DEngine::new_with_client(
            debot_addr.clone(),
            None,
            self.client.clone(),
            self.callbacks.clone()
        );
        let info: DebotInfo = dengine.init().await?.into();
        let abi_version = info.dabi_version.clone();
        let abi_ref = info.dabi.as_ref();
        let abi = load_abi(&abi_ref.ok_or(format!("DeBot ABI is not defined"))?)?;
        
        if call_start {
            let mut run_debot = autorun;
            if !autorun {
                Self::print_info(&info);
                let _ = terminal_input("Run the DeBot (y/n)?", |val| {
                    run_debot = match val.as_str() {
                        "y" => true,
                        "n" => false,
                        _ => Err(format!("invalid enter"))?,
                    };
                    Ok(())
                });
            }
            if !run_debot {
                return Err(format!("DeBot rejected"));
            }
            dengine.start().await?;
        }

        callbacks_ref.take_messages(&mut self.msg_queue);

        self.bots.insert(
            debot_addr,
            DebotEntry {
                abi,
                dengine,
                callbacks: callbacks_ref,
                info,
            }
        );
        Ok(abi_version)
    }

    async fn call_interface(
        &mut self,
        msg: String,
        interface_id: &String,
        debot_addr: &str,
    ) -> Result<(), String> {
        let debot = self
            .bots
            .get_mut(debot_addr)
            .ok_or_else(|| "Internal browser error: debot not found".to_owned())?;
        if let Some(result) = self.interfaces.try_execute(&msg, interface_id, &debot.info.dabi_version).await {
            let (func_id, return_args) = result?;
            let call_set = match func_id {
                0 => None,
                _ => CallSet::some_with_function_and_input(&format!("0x{:x}", func_id), return_args),
            };
            let response_msg = encode_internal_message(
                self.client.clone(),
                ParamsOfEncodeInternalMessage {
                    abi: Some(debot.abi.clone()),
                    address: Some(debot_addr.to_owned()),
                    call_set,
                    value: "1000000000000000".to_owned(),
                    ..Default::default()
                }
            )
            .await
            .map_err(|e| format!("{}", e))?
            .message;
            let result = debot.dengine.send(response_msg).await;
            debot.callbacks.take_messages(&mut self.msg_queue);
            if let Err(e) = result {
                println!("Debot error: {}", e);
            }
        }

        Ok(())
    }

    async fn call_debot(&mut self, addr: &str, msg: String) -> Result<(), String> {
        if self.bots.get_mut(addr).is_none() {
            self.fetch_debot(addr, false, !self.interactive).await?;
        }
        let debot = self
            .bots
            .get_mut(addr)
            .ok_or_else(|| "Internal error: debot not found")?;
        debot
            .dengine
            .send(msg)
            .await
            .map_err(|e| format!("Debot failed: {}", e))?;
        debot.callbacks.take_messages(&mut self.msg_queue);
        Ok(())
    }

    pub async fn run_message_loop(&mut self) -> Result<(), String> {
        let mut next_msg = self.msg_queue.pop_front();
        while let Some(msg) = next_msg {
            let parsed = parse_message(self.client.clone(), ParamsOfParse { boc: msg.clone() })
                .await
                .map_err(|e| format!("{}", e))?
                .parsed;

            let msg_dest = parsed["dst"]
                .as_str()
                .ok_or(format!("invalid message in queue: no dst address"))?;

            let msg_src = parsed["src"]
                .as_str()
                .ok_or(format!("invalid message in queue: no src address"))?;

            let wc_and_addr: Vec<_> = msg_dest.split(':').collect();
            let id = wc_and_addr[1].to_string();
            let wc = i8::from_str_radix(wc_and_addr[0], 10).map_err(|e| format!("{}", e))?;

            if wc == DEBOT_WC {
                if id == BROWSER_ID {
                    info!("Message from DeBot to Browser"); 
                    self.set_exit_arg(msg, msg_src).await?;
                } else {
                    self.call_interface(msg, &id, msg_src).await?;
                }
            } else {
                self.call_debot(msg_dest, msg).await?;
            }

            next_msg = self.msg_queue.pop_front();
        }
        Ok(())
    }

    pub async fn run_manifest(&mut self, manifest: DebotManifest) -> Result<Option<serde_json::Value>, String> {
        let (start, call_set, mut init_message) = {
            let mut processor = self.processor.write().await;
            processor.load_manifest(manifest);
            ( 
                processor.default_start(),
                processor.initial_call_set(),
                processor.initial_msg()
            )
        };
        
        self.exit_arg = None;

        let abi = self.bots.get(&self.main_debot_addr)
            .ok_or_else(|| format!("Starting DeBot not found: {}", &self.main_debot_addr))?
            .abi.clone();

        if !start && init_message.is_none() {
            init_message = Some(
                encode_internal_message(
                    self.client.clone(),
                    ParamsOfEncodeInternalMessage {
                        abi: Some(abi),
                        address: Some(self.main_debot_addr.clone()),
                        src_address: Some(format!("{}:{}", DEBOT_WC, BROWSER_ID)),
                        call_set,
                        value: "1000000000000000".to_owned(),
                        ..Default::default()
                    },
                )
                .await
                .map_err(|e| format!("{}", e))?
                .message,
            );
        }

        if let Some(msg) = init_message {
            let addr = self.main_debot_addr.clone();
            self.call_debot(&addr, msg).await?;
        }

        self.run_message_loop().await?;

        Ok(self.exit_arg.clone())
    }

    fn print_info(info: &DebotInfo) {
        println!("DeBot Info:");
        fn print<'a>(field: &'a Option<String>) -> &'a str {
            field.as_ref().map(|v| v.as_str()).unwrap_or("None")
        } 
        println!("Name   : {}", print(&info.name));
        println!("Version: {}", print(&info.version));
        println!("Author : {}", print(&info.author));
        println!("Publisher: {}", print(&info.publisher));
        println!("Support: {}", print(&info.support));
        println!("Description: {}", print(&info.caption));
        println!("{}", print(&info.hello));
    }

    async fn set_exit_arg(&mut self, message: String, _debot_addr: &str) -> Result<(), String> {
        let abi = self.processor.read().await.abi();
        let arg = if let Some(abi) = abi {
            let decoded = decode_message(
                self.client.clone(),
                ParamsOfDecodeMessage { abi, message },
            ).await.map_err(|e| format!("{}", e))?;
            decoded.value.unwrap_or(json!({}))
        } else {
            json!({"message": message})
        };
        self.exit_arg = Some(arg);
        Ok(())
    }
}

pub(crate) fn input<R, W>(prefix: &str, reader: &mut R, writer: &mut W) -> String
where
    R: BufRead,
    W: Write,
{
    let mut input_str = "".to_owned();
    let mut argc = 0;
    while argc == 0 {
        println!("{}", prefix);
        if let Err(e) = writer.flush() {
            println!("failed to flush: {}", e);
            return input_str;
        }
        if let Err(e) = reader.read_line(&mut input_str) {
            println!("failed to read line: {}", e);
            return input_str;
        }
        argc = input_str
            .split_whitespace()
            .map(|x| x.parse::<String>().unwrap())
            .collect::<Vec<String>>()
            .len();
    }
    input_str.trim().to_owned()
}

pub(crate) fn terminal_input<F>(prompt: &str, mut validator: F) -> String
where
    F: FnMut(&String) -> Result<(), String>,
{
    let stdio = io::stdin();
    let mut reader = stdio.lock();
    let mut writer = io::stdout();
    let mut value = input(prompt, &mut reader, &mut writer);
    while let Err(e) = validator(&value) {
        println!("{}. Try again.", e);
        value = input(prompt, &mut reader, &mut writer);
    }
    value
}
pub fn action_input(max: usize) -> Result<(usize, usize, Vec<String>), String> {
    let mut a_str = String::new();
    let mut argc = 0;
    let mut argv = vec![];
    println!();
    while argc == 0 {
        print!("debash$ ");
        let _ = io::stdout().flush();
        io::stdin()
            .read_line(&mut a_str)
            .map_err(|e| format!("failed to read line: {}", e))?;
        argv = a_str
            .split_whitespace()
            .map(|x| x.parse::<String>().expect("parse error"))
            .collect::<Vec<String>>();
        argc = argv.len();
    }
    let n = usize::from_str_radix(&argv[0], 10)
        .map_err(|_| format!("Oops! Invalid action. Try again, please."))?;
    if n > max {
        Err(format!("Auch! Invalid action. Try again, please."))?;
    }

    Ok((n, argc, argv))
}



#[cfg(test)]
mod tests {}
