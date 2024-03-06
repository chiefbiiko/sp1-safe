//! A simple script to generate and verify the proof of a given program.

use sp1_core::{SP1Prover, SP1Stdin, SP1Verifier};
use sp1_safe_basics::Inputs;
use const_hex;
mod util;
use util::fetch_inputs;

const ELF: &[u8] = include_bytes!("../../program/elf/riscv32im-succinct-zkvm-elf");

fn bytes20(x: Vec<u8>) -> [u8; 20] {
    x.try_into().expect("invalid address")
}

fn bytes32(x: Vec<u8>) -> [u8; 32] {
    x.try_into().expect("invalid hash")
}

#[tokio::main]
async fn main() {
    // Assemble and write inputs
    let mut stdin = SP1Stdin::new();
    let safe = bytes20(const_hex::decode(std::env::var("SAFE").expect("must set env var SAFE=0x...")).expect("not hex"));
    let msg_hash = bytes32(const_hex::decode(std::env::var("MSG_HASH").expect("must set env var MSG_HASH=0x...")).expect("not hex"));
    let inputs = fetch_inputs(safe.into(), msg_hash.into()).await;
    stdin.write::<Inputs>(&inputs);

    // Generate proof
    // NOTE only executing instead of proving/verifying while dev
    // let mut proof = SP1Prover::prove(ELF, stdin).expect("proving failed");
    // let a = proof.stdout.read::<u128>();
    // let b = proof.stdout.read::<u128>();
    let mut stdout = SP1Prover::execute(ELF, stdin).expect("execution failed");
    let state_root = stdout.read::<[u8; 32]>(); 
    let blind_safe = stdout.read::<[u8; 32]>();

    println!("state root: {:02X?}", state_root);
    println!("blind safe: {:02X?}", blind_safe);

    // // Verify and save proof
    // SP1Verifier::verify(ELF, &proof).expect("verification failed");
    // proof
    //     .save("proof-with-io.json")
    //     .expect("saving proof failed");

    println!("sp1 program ok");
}
