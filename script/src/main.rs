//! A simple script to generate the proof of the sp1-safe program.

use const_hex;
use sp1_core::{SP1Prover, SP1Stdin /*, SP1Verifier*/};
use sp1_safe_basics::{coerce_bytes20, coerce_bytes32, fetch_inputs, Inputs};

const ELF: &[u8] = include_bytes!("../../program/elf/riscv32im-succinct-zkvm-elf");

#[tokio::main]
async fn main() {
    sp1_core::utils::setup_logger();
    // Assemble and write inputs
    let rpc = std::env::var("RPC").unwrap_or("https://rpc.gnosis.gateway.fm".to_string());
    let safe = coerce_bytes20(
        const_hex::decode(std::env::var("SAFE").expect("must set env var SAFE=0x..."))
            .expect("not hex"),
    );
    let msg_hash = coerce_bytes32(
        const_hex::decode(std::env::var("MSG_HASH").expect("must set env var MSG_HASH=0x..."))
            .expect("not hex"),
    );
    let inputs = fetch_inputs(&rpc, safe.into(), msg_hash.into()).await;
    let mut stdin = SP1Stdin::new();
    stdin.write::<Inputs>(&inputs);

    // Generate proof
    // let mut proof = SP1Prover::prove(ELF, stdin).expect("proving failed");
    let mut stdout = SP1Prover::execute(ELF, stdin).expect("execution failed");

    let blockhash = stdout.read::<[u8; 32]>();
    let challenge = stdout.read::<[u8; 32]>();
    println!(
        "safe multisig storage proof ok:\nblockhash={}\nchallenge={}",
        const_hex::encode(blockhash),
        const_hex::encode(challenge)
    );

    // Verify and save proof
    // SP1Verifier::verify(ELF, &proof).expect("verification failed");
    // proof
    //     .save("proof-with-io.json")
    //     .expect("saving proof failed");
}
