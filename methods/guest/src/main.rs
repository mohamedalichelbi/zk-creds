#![no_main]


extern crate alloc;

use risc0_zkvm::{
    guest::env,
    sha::{self, Sha256},
};
use core::types::{ZkCommit, ScriptLang};
use rhai::Engine;
use boa_engine::{Context, Source};
use serde_json::{Value, de::from_str};
use alloc::string::String;
use base64ct::{Base64, Encoding};

risc0_zkvm::guest::entry!(main);

pub fn main() {
    // error flag
    let mut has_error = false;
    
    // get credentials
    let credentials: Vec<String> = env::read();
    // get script
    let script_lang: ScriptLang = env::read();
    // get script
    let input_script: String = env::read();

    // validate that credentials are JSON objects
    for cred in credentials.iter() {
        let a = from_str::<Value>(cred);
        if a.is_err() { has_error = true; break; }
        else {
            match a.unwrap() {
                Value::Object(_) => (),
                _ => { has_error = true; break; }
            }   
        }
    }

    // stop if we found any errors
    if has_error {
        env::commit(&ZkCommit {
            has_error: true,
            err_msg: "failed to parse credentials".to_string(),
            cred_hashes: Vec::new(),
            script: input_script,
            result: false,
        });
        
        return;
    }

    // calculate sha256 hash of each credential
    let cred_hashes: Vec<String> = credentials
        .iter()
        .map(|cred| Base64::encode_string(sha::Impl::hash_bytes(cred.as_bytes()).as_bytes()))
        .collect();

    
    match script_lang {
        ScriptLang::Rhai => {
            // inject credentials in the script
            // 1- object maps in Rhai start with "#"
            let cred_strings = credentials
            .iter()
            .map(|cred_str| {
                let mut rhai_obj_str = "#".to_string();
                rhai_obj_str.push_str(cred_str);
                rhai_obj_str
            })
            .collect::<Vec<String>>();

            // 2- add array variable "credentials" to hold all cred objects
            let mut script = "let credentials = [".to_string();
            script.push_str(&cred_strings.join(","));
            script.push_str("]; ");

            // 3- add the rest of the program
            script.push_str(&input_script);

            let engine = Engine::new_raw();

            let raw_result = engine.eval::<bool>(&script);

            if raw_result.is_err() {
                env::commit(&ZkCommit {
                    has_error: true,
                    err_msg: format!("script error: {}", raw_result.err().unwrap()),
                    cred_hashes,
                    script: input_script,
                    result: false,
                });
                
                return;
            }

            env::commit(&ZkCommit {
                has_error: false,
                err_msg: "".to_string(),
                cred_hashes,
                // IMPORTANT!! use input script here to not expose credentials
                script: input_script,
                result: raw_result.unwrap(),
            });
        },
        ScriptLang::JavaScript => {
            // inject credentials in the script
            // 1- add array variable "credentials" to hold all cred objects
            let mut script = "let credentials = [".to_string();
            script.push_str(&credentials.join(","));
            script.push_str("]; ");

            // 2- add the rest of the program
            script.push_str(&input_script);
            // Instantiate the execution context
            let mut context = Context::default();

            // Parse the source code
            let source = Source::from_bytes(&script);
            match context.eval(source) {
                Ok(res) => {
                    let bool_res = res.as_boolean();
                    if bool_res.is_none() {
                        env::commit(&ZkCommit {
                            has_error: true,
                            err_msg: "script error: result not boolean".to_string(),
                            cred_hashes,
                            script: input_script,
                            result: false,
                        });    
                    }
                    else {
                        let result = bool_res.unwrap();
                        env::commit(&ZkCommit {
                            has_error: false,
                            err_msg: "".to_string(),
                            cred_hashes,
                            // IMPORTANT!! use input script here to not expose credentials
                            script: input_script,
                            result,
                        });

                    }
                }
                Err(e) => {
                    env::commit(&ZkCommit {
                        has_error: true,
                        err_msg: format!("script error: {}", e),
                        cred_hashes,
                        script: input_script,
                        result: false,
                    });
                }
            };
        },
    }
}
