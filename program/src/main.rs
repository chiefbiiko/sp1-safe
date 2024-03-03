//! Prove a storage value in zk (non-doxing) in the zkVM.

#![no_main]
sp1_zkvm::entrypoint!(main);

// use alloy_rlp::{encode, encode_list, Decodable, Encodable, Error, Header};
use tiny_keccak::Keccak;

fn keccak256_2(a: &[u8], b: &[u8], out: &mut [u8; 32]) {
    let mut keccak256 = Keccak::v256();
    keccak256.update(a);
    keccak256.update(b);
    keccak256.finalize(&mut out);
}

// eth_getProof
// Returns the account and storage values, including the Merkle proof, of the specified account.
// Parameters
//     address: Safe
//     storageKeys: [keccak256(stxfp . uint256(5))] //5=signedMessages
//     blockParameter: A hexadecimal block number, or the string "latest" or"earliest". See the default block parameter.
// Returns
//     balance: Hexadecimal of the current balance in wei.
//     codeHash: The 32-byte hash of the code of the account.
//     nonce: The nonce of the account.
//     storageHash: 32 bytes. The SHA3 of the StorageRoot. All storage will deliver a Merkle proof starting with this rootHash.
//     accountProof: An array of RLP-serialized MerkleTree-Nodes, starting with the stateRoot-Node, following the path of the SHA3 (address) as key.
//     storageProof: An array of storage-entries as requested. Each entry is an object with these properties:
//         key: The requested storage key.
//         value: The storage value.
//         proof: An array of RLP-serialized MerkleTree-Nodes, starting with the storageHash-Node, following the path of the SHA3 (key) as path.

// https://github.com/polytope-labs/solidity-merkle-trees/blob/f637bb083eb6bdef3daee016e300a56800c2725e/src/MerklePatricia.sol#L133
// /**
//  * @notice Verifies ethereum specific merkle patricia proofs as described by EIP-1188.
//     * @param root hash of the merkle patricia trie
//     * @param proof a list of proof nodes
//     * @param keys a list of keys to verify
//       @param expectedValue 
//     * @return bytes[] a list of values corresponding to the supplied keys.
//     */
// function VerifyEthereumProof(bytes32 root, bytes[] memory proof, bytes[] memory keys)

pub fn main() {

    // read inputs
    // blockhash        0:32
    // storageNodes     32:
    // TODO rest of raw storage proof 32:
    // TODO add raw header 2 recalc blockhash

                

    // prove storage root recalcable given storageKey and siblings

}
