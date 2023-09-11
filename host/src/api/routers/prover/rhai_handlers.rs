use methods::{ZK_PROVER_ELF, ZK_PROVER_ID};
use core::types::ZkCommit;
use risc0_zkvm::{
    Executor, ExecutorEnv, ExecutorEnvBuilder,
    serde::{to_vec, from_slice},
};
use std::time::Instant;
use rhai::Engine;
use axum::{Json, http::StatusCode};
use serde::{Serialize, Deserialize};
use serde_json::ser::to_string;
use base64ct::{Base64, Encoding};


#[derive(Serialize)]
pub struct GenProofResponse {
    proof: String,
}

#[derive(Deserialize)]
pub struct GenProofArgs {
    credentials: Vec<String>,
    script: String,
}

// basic handler that responds with a static string
pub async fn gen_rhai_proof(Json(payload): Json<GenProofArgs>) -> (StatusCode, Json<GenProofResponse>) {
    /* Example script
    "let credential = #{
        "age": 19,
    };
    
    fn check_age(cred) {
        return cred["age"] > 18;
    }
    
    check_age(credential);"
    */
    
    //let rhai_engine = Engine::new_raw();
    //let rhai_ast = rhai_engine.compile(rhai_code).unwrap();

    // First, we construct an executor environment
    let env = ExecutorEnv::builder()
        .add_input(&to_vec(&payload.credentials).unwrap())
        .add_input(&to_vec(&payload.script).unwrap())
        .build()
        .unwrap();

    // Next, we make an executor, loading the (renamed) ELF binary.
    let mut exec = Executor::from_elf(env, ZK_PROVER_ELF).unwrap();

    // Run the executor to produce a session.
    let session = exec.run().unwrap();

    let start_time_prover = Instant::now();

    // Prove the session to produce a receipt.
    let receipt = session.prove().unwrap();

    println!("Prover duration {:?}", start_time_prover.elapsed());
    println!("Receipt size {:.2} (KB)", (to_vec(&receipt).unwrap().len() / 1024));

    // Get guest result
    let code_result: ZkCommit = from_slice(&receipt.journal).unwrap();
    println!("Result: {:?}", to_string(&code_result));

    // Verify receipt to confirm that recipients will also be able to verify it
    let start_time_verifier = Instant::now();
    receipt.verify(ZK_PROVER_ID).unwrap();
    println!("Verifier duration {:?}", start_time_verifier.elapsed());

    (
        StatusCode::ACCEPTED,
        Json(GenProofResponse {
            proof: Base64::encode_string(&bincode::serialize(&receipt).unwrap()),
        })
    )
}
