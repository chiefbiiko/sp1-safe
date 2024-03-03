//! A simple program to be proven inside the zkVM.

#![no_main]
sp1_zkvm::entrypoint!(main);

use tiny_keccak::Keccak;

fn keccak256_2(a: &[u8], b: &[u8], out: &mut [u8; 32]) {
    let mut keccak256 = Keccak::v256();
    keccak256.update(a);
    keccak256.update(b);
    keccak256.finalize(&mut out);
}
//HASHING
// let mut keccak256 = Keccak::v256();
// let mut output = [0u8; 32];
// keccak256.update(b"hello");
// keccak256.update(b" ");
// keccak256.update(b"world");
// keccak256.finalize(&mut output);

pub fn main() {

    // let scratch_root...
    // let rolling_root...



}
