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
use super::dinterface::{decode_answer_id, decode_prompt, decode_string_arg};
use super::menu::{MenuItem, ID as MENU_ID};
use super::terminal::ID as TERMINAL_ID;
use crate::{ChainProcessor, ProcessorError};
use serde_json::{json, Value};
use std::sync::Arc;
use tokio::sync::RwLock;
use ton_client::abi::Abi;
use ton_client::debot::{DebotInterface, InterfaceResult};

pub struct InputInterface {
    processor: Arc<RwLock<ChainProcessor>>,
    inner_interface: Arc<dyn DebotInterface + Send + Sync>,
}

impl InputInterface {
    pub fn new(
        inner_interface: Arc<dyn DebotInterface + Send + Sync>,
        processor: Arc<RwLock<ChainProcessor>>,
    ) -> Self {
        Self {
            inner_interface,
            processor,
        }
    }
}

#[async_trait::async_trait]
impl DebotInterface for InputInterface {
    fn get_id(&self) -> String {
        self.inner_interface.get_id()
    }

    fn get_abi(&self) -> Abi {
        self.inner_interface.get_abi()
    }

    async fn call(&self, func: &str, args: &Value) -> InterfaceResult {
        if &self.get_id() == TERMINAL_ID {
            if func == "print" || func == "printf" {
                return self.inner_interface.call(func, args).await;
            }
        }
        let result = self
            .processor
            .write()
            .await
            .next_input(&self.get_id(), func, args);
        match result {
            Err(ProcessorError::InterfaceCallNeeded) => {
                let res = self.inner_interface.call(func, args).await?;
                Ok(res)
            }
            Err(e) => Err(format!("{:?}", e))?,
            Ok(params) => {
                let prompt = decode_prompt(args);
                let title = decode_string_arg(args, "title");
                let processor = self.processor.read().await;
                if let Ok(prompt) = prompt {
                    processor.print(&prompt);
                }
                if let Ok(prompt) = title {
                    processor.print(&prompt);
                }
                let params = params.unwrap_or(json!({}));
                if let Some(args) = params.as_object() {
                    for arg in args {
                        processor.print(&format!("{}", arg.1));
                    }
                }
                let answer_id = if self.get_id() == MENU_ID {
                    let n = params["index"]
                        .as_u64()
                        .ok_or(format!("invalid arguments for menu callback"))?;
                    let menu_items: Vec<MenuItem> =
                        serde_json::from_value(args["items"].clone()).map_err(|e| e.to_string())?;
                    let menu = menu_items.get(n as usize);
                    menu.ok_or(format!("menu index is out of range"))?
                        .handler_id
                } else {
                    decode_answer_id(args)?
                };
                Ok((answer_id, params))
            }
        }
    }
}
