#![no_main]


extern crate alloc;

use risc0_zkvm::guest::env;
use core::types::ZkCommit;
use rhai::Engine;
use serde_json::{
    Value,
    de::from_str,
    ser::to_string,
};
use alloc::string::String;

risc0_zkvm::guest::entry!(main);

pub fn main() {
    // error flag
    let mut has_error = false;
    
    // get credentials
    let credentials: Vec<String> = env::read();
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
            script: input_script,
            result: false,
        });
        
        return;
    }

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
    script.push_str(&cred_strings.join(""));
    script.push_str("]; ");

    // 3- add the rest of the program
    script.push_str(&input_script);

    let engine = Engine::new_raw();
    
    let raw_result = engine.eval::<bool>(&script);

    if raw_result.is_err() {
        env::commit(&ZkCommit {
            has_error: true,
            err_msg: "script error".to_string(),
            script: input_script,
            result: false,
        });
        
        return;
    }

    env::commit(&ZkCommit {
        has_error: false,
        err_msg: "".to_string(),
        // IMPORTANT!! use input script here to not expose credentials
        script: input_script,
        result: raw_result.unwrap(),
    });
}
