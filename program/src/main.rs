//! Prove a storage value in zk (non-doxing) in the zkVM.

#![no_main]
sp1_zkvm::entrypoint!(main);

use tiny_keccak::Keccak;

fn keccak256_2(a: &[u8], b: &[u8], out: &mut [u8; 32]) {
    let mut keccak256 = Keccak::v256();
    keccak256.update(a);
    keccak256.update(b);
    keccak256.finalize(&mut out);
}

pub fn main() {

    // let scratch_root...
    // let rolling_root...


    // read inputs
    // blockhash      0:32
    //   TODO add all components to recalc blockhash
    // address        32:52
    //   nonce        52:84
    //   balance      84:116
    //   storageRoot  116:148
    //   codeHash     148:180
    
    // storageKey     180:212
    // storageValue   212:244
    // siblings       244...
                

    // prove storage root recalcable given storageKey and siblings

}
