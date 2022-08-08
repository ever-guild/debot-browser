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
use serde_json::{Value, json};
use ton_client::abi::Abi;
use ton_client::debot::{DebotInterface, InterfaceResult};

const ECHO_ID: &'static str = "f6927c0d4bdb69e1b52d27f018d156ff04152f00558042ff674f0fec32e4369d";

pub const ECHO_ABI: &str = r#"
{
	"ABI version": 2,
	"header": ["time"],
	"functions": [
		{
			"name": "echo",
			"inputs": [
				{"name":"answerId","type":"uint32"},
				{"name":"request","type":"bytes"}
			],
			"outputs": [
				{"name":"response","type":"bytes"}
			]
		},
		{
			"name": "constructor",
			"inputs": [
			],
			"outputs": [
			]
		}
	],
	"data": [
	],
	"events": [
	]
}
"#;

pub struct Echo {}
impl Echo {
	pub fn new() -> Self {
		Self {}
	}

	fn echo(&self, args: &Value) -> InterfaceResult {
		let answer_id = u32::from_str_radix(args["answerId"].as_str().unwrap(), 10).unwrap();
		let request_vec = hex::decode(args["request"].as_str().unwrap()).unwrap();
		let request = std::str::from_utf8(&request_vec).unwrap();
		Ok((
			answer_id,
			json!({ "response": hex::encode(request.as_bytes()) }),
		))
	}
}

#[async_trait::async_trait]
impl DebotInterface for Echo {
	fn get_id(&self) -> String {
		ECHO_ID.to_string()
	}

	fn get_abi(&self) -> Abi {
		Abi::Json(ECHO_ABI.to_owned())
	}

	async fn call(&self, func: &str, args: &Value) -> InterfaceResult {
		match func {
			"echo" => self.echo(args),
			_ => Err(format!("function \"{}\" is not implemented", func)),
		}
	}
}
