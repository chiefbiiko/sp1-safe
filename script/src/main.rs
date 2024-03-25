//! A simple script to generate the proof of the sp1-safe program.

use const_hex;
use sp1_core::{SP1Prover, SP1Stdin /*, SP1Verifier*/};
use sp1_safe_basics::Inputs;
use sp1_safe_fetch::fetch_inputs;

const ELF: &[u8] = include_bytes!("../../program/elf/riscv32im-succinct-zkvm-elf");

#[tokio::main]
async fn main() {
    sp1_core::utils::setup_logger();
    // Assemble and write inputs
    let rpc = std::env::var("RPC").unwrap_or("https://rpc.gnosis.gateway.fm".to_string());
    let safe = const_hex::decode_to_array::<&str, 20>(
        &std::env::var("SAFE").expect("must set env var SAFE=0x..."),
    )
    .expect("env var SAFE");
    let msg_hash = const_hex::decode_to_array::<&str, 32>(
        &std::env::var("MSG_HASH").expect("must set env var MSG_HASH=0x..."),
    )
    .expect("env var MSG_HASH");
    let (anchor, inputs) = fetch_inputs(&rpc, safe.into(), msg_hash.into())
        .await
        .expect("fetch_inputs failed");
    let mut stdin = SP1Stdin::new();
    stdin.write::<Inputs>(&inputs);

    // Generate proof
    // let mut proofwio = SP1Prover::prove(ELF, stdin).expect("proving failed");
    let mut stdout = SP1Prover::execute(ELF, stdin).expect("execution failed");

    let blockhash = stdout.read::<[u8; 32]>();
    let challenge = stdout.read::<[u8; 32]>();

    println!(
        "Safe multisig storage proof verification passed
=== proof params ===
Safe address: 0x{}
msg hash: 0x{}
block number: {}
=== proof outputs ===
blockhash: 0x{}
challenge: 0x{}",
        // proof: 0x{}",
        const_hex::encode(&safe),
        const_hex::encode(&msg_hash),
        anchor,
        const_hex::encode(&blockhash),
        const_hex::encode(&challenge),
        // const_hex::encode(bincode::serialize(&proofwio.proof).expect("bincode")),
    );

    // Verify and save proof
    // SP1Verifier::verify(ELF, &proof).expect("verification failed");
    // proof
    //     .save("proof-with-io.json")
    //     .expect("saving proof failed");
}
