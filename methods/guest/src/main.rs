#![no_main]


extern crate alloc;

use risc0_zkvm::guest::env;
use rhai::Engine;
use alloc::string::String;

risc0_zkvm::guest::entry!(main);

pub fn main() {
    let rhai_code: String = env::read();

    let engine = Engine::new_raw();

    let result: bool = engine.eval(&rhai_code).unwrap();

    env::commit(&result);
}
