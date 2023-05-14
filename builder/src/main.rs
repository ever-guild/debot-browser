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
*
*/

use regex::Regex;
use std::fs;
use std::process::{Command, ExitStatus};

const CRATE_NAME: &str = "debot-browser";

fn fix_wat(mut wrapper: String) -> String {
    let patterns = [
        r#"(?m)^\s*\(import\s+"env"\s+"malloc"\s+\(func \(;(?P<f>\d+);\)\s+\(type (?P<t>\d+)\)\)\)"#,
        r#"(?m)^\s*\(import\s+"env"\s+"free"\s+\(func \(;(?P<f>\d+);\)\s+\(type (?P<t>\d+)\)\)\)"#,
        r#"(?m)^\s*\(import\s+"env"\s+"now"\s+\(func \(;(?P<f>\d+);\)\s+\(type (?P<t>\d+)\)\)\)"#,
    ];
    for p in patterns {
        let reg = Regex::new(p).unwrap();
        if reg.is_match(&wrapper) {
            reg.captures(&wrapper).unwrap();
            wrapper = reg
            .replace_all(
                &wrapper,
                format!(
                    r#"  (import "./{}_bg.js" "__wbg_msCrypto_9ad6677321a08dd8" (func (;$f;) (type $t)))"#,
                    CRATE_NAME.replace('-', "_")
                ),
            )
            .to_string();
        }
    }

    wrapper
}

fn main() {
    assert!(exec(
        "wasm-pack",
        &["build", CRATE_NAME, "--release", "--scope", "ever-guild",]
    )
    .success());

    let repo_dir = std::env::current_dir().unwrap();
    let crate_dir = repo_dir.join(CRATE_NAME);
    let pkg_dir = crate_dir.join("pkg");

    let wasm_path = pkg_dir.join(format!("{}_bg.wasm", CRATE_NAME).replace('-', "_"));

    println!("Disassembling wasm...");
    let wat = wasmprinter::print_file(&wasm_path).unwrap();
    println!("Fixing...");
    let fixed_wat = fix_wat(wat);
    //fs::write(&pkg_dir.join(format!("{}.wat", CRATE_NAME)), fixed_wat).unwrap();
    println!("Building wasm...");
    let fixed_wasm = wat::parse_str(fixed_wat).unwrap();
    println!("Done.");
    fs::write(&wasm_path, fixed_wasm).unwrap();
}

pub fn exec(cmd: &str, args: &[&str]) -> ExitStatus {
    Command::new(cmd)
        .args(args)
        .spawn()
        .unwrap()
        .wait()
        .unwrap()
}
