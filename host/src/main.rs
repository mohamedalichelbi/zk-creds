// TODO: Update the name of the method loaded by the prover. E.g., if the method
// is `multiply`, replace `METHOD_NAME_ELF` with `MULTIPLY_ELF` and replace
// `METHOD_NAME_ID` with `MULTIPLY_ID`
use methods::{PROVE_JS_ELF, PROVE_JS_ID};
use risc0_zkvm::{
    Executor, ExecutorEnv, ExecutorEnvBuilder,
    serde::{to_vec, from_slice},
};
use std::time::Instant;
use rhai::Engine;

fn main() {
    let rhai_code = r#"
    let credential = #{
        "age": 19,
    };
    
    fn check_age(cred) {
        return cred["age"] > 18;
    }
    
    check_age(credential);
    "#;
    let rhai_engine = Engine::new_raw();
    let rhai_ast = rhai_engine.compile(rhai_code).unwrap();

    // First, we construct an executor environment
    let env = ExecutorEnv::builder()
        .add_input(&to_vec(&rhai_code).unwrap())
        .build()
        .unwrap();

    // Next, we make an executor, loading the (renamed) ELF binary.
    let mut exec = Executor::from_elf(env, PROVE_JS_ELF).unwrap();

    // Run the executor to produce a session.
    let session = exec.run().unwrap();

    let start_time_prover = Instant::now();

    // Prove the session to produce a receipt.
    let receipt = session.prove().unwrap();

    println!("Prover duration {:?}", start_time_prover.elapsed());
    println!("Receipt size {:.2} (KB)", (to_vec(&receipt).unwrap().len() / 1024));

    // Get guest result
    let code_result: bool = from_slice(&receipt.journal).unwrap();
    println!("Result: {:?}", code_result);

    // TODO: Implement code for transmitting or serializing the receipt for
    // other parties to verify here

    // Optional: Verify receipt to confirm that recipients will also be able to
    // verify your receipt
    receipt.verify(PROVE_JS_ID).unwrap();
}
