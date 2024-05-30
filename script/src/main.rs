//! A simple script to generate the proof of the sp1-safe program.

use const_hex;
use serde_json::json;
use sp1_safe_basics::{Inputs, Sp1SafeResult};
use sp1_safe_fetch::fetch_inputs;
use sp1_sdk::{ProverClient, SP1Stdin};

const ELF: &[u8] = include_bytes!("../../program/elf/riscv32im-succinct-zkvm-elf");

#[tokio::main]
async fn main() {
    sp1_sdk::utils::setup_logger();
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
    let client = ProverClient::new();
    let (pk, _vk) = client.setup(ELF);
    let mut proofwpv = client.prove_plonk(&pk, stdin).expect("proving failed");

    let blockhash = proofwpv.public_values.read::<[u8; 32]>();
    let challenge = proofwpv.public_values.read::<[u8; 32]>();

    println!(
        "{}",
        json!(Sp1SafeResult {
            chain_id: 100,
            safe_address: format!("0x{}", const_hex::encode(safe)),
            message_hash: format!("0x{}", const_hex::encode(msg_hash)),
            block_number: anchor,
            block_hash: format!("0x{}", const_hex::encode(blockhash)),
            challenge: format!("0x{}", const_hex::encode(challenge)),
            proof: format!(
                "0x{}",
                const_hex::encode(bincode::serialize(&proofwpv.proof).expect("bincode"))
            ),
        })
        .to_string()
    );

    // // Verify proof and public values
    // client
    //     .verify_groth16(&proofwpv, &vk)
    //     .expect("verification failed");
}
