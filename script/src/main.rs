//! A simple script to generate and verify the proof of a given program.

use sp1_core::{SP1Prover, SP1Stdin, SP1Verifier};

const ELF: &[u8] = include_bytes!("../../program/elf/riscv32im-succinct-zkvm-elf");

fn main() {
    // Generate proof.
    let mut stdin = SP1Stdin::new();
    let n = 186u32;
    stdin.write(&n);
    // NOTE only executing instead of proving/verifying while dev
    // let mut proof = SP1Prover::prove(ELF, stdin).expect("proving failed");
    // let a = proof.stdout.read::<u128>();
    // let b = proof.stdout.read::<u128>();
    let mut stdout = SP1Prover::execute(ELF, stdin).expect("execution failed");
    let a = stdout.read::<u32>(); 
    let b = stdout.read::<u32>();

    println!("a: {}", a);
    println!("b: {}", b);

    // // Verify and save proof.
    // SP1Verifier::verify(ELF, &proof).expect("verification failed");
    // proof
    //     .save("proof-with-io.json")
    //     .expect("saving proof failed");

    println!("sp1 program ok");
}
