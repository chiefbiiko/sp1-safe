use const_hex;
use sp1_sdk::{HashableKey, ProverClient};
use std::path::PathBuf;

const ELF: &[u8] = include_bytes!("../../program/elf/riscv32im-succinct-zkvm-elf");

fn main() {
    let client = ProverClient::new();
    let (_pk, vk) = client.setup(ELF);

    sp1_sdk::artifacts::export_solidity_plonk_bn254_verifier(PathBuf::from(env!("CARGO_MANIFEST_DIR")))
        .expect("failed to export verifier");

    println!("✞✞✞✞✞ vk {}", const_hex::encode(&vk.hash_bytes()));
}
