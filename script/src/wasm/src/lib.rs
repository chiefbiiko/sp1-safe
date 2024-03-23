use sp1_core::{SP1Prover, SP1Stdin};
use wasm_bindgen::prelude::*;

const ELF: &[u8] = include_bytes!("../../../../program/elf/riscv32im-succinct-zkvm-elf");

#[wasm_bindgen]
pub struct Wrapper {
    blockhash: Vec<u8>,
    challenge: Vec<u8>,
    proof: Vec<u8>,
}

#[wasm_bindgen]
impl Wrapper {
    #[wasm_bindgen(constructor)]
    pub fn new(blockhash: Vec<u8>, challenge: Vec<u8>, proof: Vec<u8>) -> Wrapper {
        Wrapper {
            blockhash,
            challenge,
            proof,
        }
    }
    pub fn blockhash(&self) -> Vec<u8> {
        self.blockhash.clone()
    }
    pub fn challenge(&self) -> Vec<u8> {
        self.challenge.clone()
    }
    pub fn proof(&self) -> Vec<u8> {
        self.proof.clone()
    }
}

#[wasm_bindgen]
pub fn prove(inputs: &[u8]) -> Wrapper {
    let mut stdin = SP1Stdin::new();
    stdin.write_slice(&inputs);

    let mut proofwio = SP1Prover::prove(ELF, stdin).expect("proving failed");
    // let mut stdout = SP1Prover::execute(ELF, stdin).expect("execution failed");

    let blockhash = proofwio.stdout.read::<[u8; 32]>();
    let challenge = proofwio.stdout.read::<[u8; 32]>();

    let wrapper = Wrapper::new(
        blockhash.to_vec(),
        challenge.to_vec(),
        bincode::serialize(&proofwio.proof).expect("serialization failed"),
    );

    return wrapper;
}

#[cfg(test)]
mod test {
    use crate::prove;
    use wasm_bindgen_test::wasm_bindgen_test;
    wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

    #[wasm_bindgen_test]
    pub fn test_prove() {
        let witness = b"";

        let wrapper = prove(witness);

        assert_eq!(wrapper.blockhash().len(), 32);
        assert_eq!(wrapper.challenge().len(), 32);
        assert!(wrapper.proof().len() > 0);
    }
}