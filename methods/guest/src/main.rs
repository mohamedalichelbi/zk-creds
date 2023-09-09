#![no_main]
//#![no_std]


extern crate alloc;

use risc0_zkvm::guest::env;
use boa_engine::{Source, Context, context::};
use alloc::string::String;

risc0_zkvm::guest::entry!(main);

pub fn main() {
    let js_code: Source<> = env::read();

    let mut js_context = Context::default();

    // Parse the source code
    match js_context.eval(Source::from_bytes(&js_code)) {
        Ok(js_res) => {
            let res = js_res.as_boolean();
            match res {
                Some(bool_res) => env::commit(&bool_res),
                None => env::commit(&false),
            }
        }
        Err(_e) => {
            env::commit(&false);
        }
    };
}
