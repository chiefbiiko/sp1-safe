// #![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;

use alloc::vec::Vec;
use alloy_trie::{HashBuilder, Nibbles};
use tiny_keccak::{Hasher, Keccak};

pub fn keccak256(input: &[u8]) -> alloy_primitives::FixedBytes<32> {
    let mut out = [0u8; 32];
    let mut k = Keccak::v256();
    k.update(input);
    k.finalize(&mut out);
    out.into()
}

// use patricia_merkle_tree::PatriciaMerkleTree;
// use sha3::Keccak256;
// use sha3::Digest;

// pub fn keccak256(input: &[u8]) -> [u8; 32] {
//     // let mut out = [0u8; 32];
//     let mut k = Keccak256::default();
//     k.update(input);
//     k.finalize().into()
// }

// pub fn mpt_root(proof: Vec<Vec<u8>>) -> Vec<u8> {
//     let mut trie = PatriciaMerkleTree::<&[u8], &[u8], Keccak256>::new();
//      let n = proof.len()
//      let i = 0
//     for node in proof {
//         let k = keccak256(&node);
//         trie.insert(
//             &k,    // hashed storage key
//             &node, // raw rlp
//         );
//     }
//     trie.compute_hash().as_slice().to_vec()
// }

pub fn mpt_root(proof: Vec<Vec<u8>>) -> [u8; 32] {
    let mut hb = HashBuilder::default();
    hb.print_stack();
    // set branch nodes
    for i in 0..(proof.len() - 2) {
        let v = &proof[i];
        let k = keccak256(v);
        let n = Nibbles::unpack(k);
        println!("setting branch");
        hb.add_branch(n, k, true); //children_are_in_trie
        hb.print_stack();
    }
    // set leaf - that is the last proof array item
    let v = &proof[proof.len() - 1];
    let k = keccak256(v);
    let n = Nibbles::unpack(k);
    println!("setting leaf");
    hb.add_leaf(n, v);
    hb.print_stack();
    // get root
    let root = hb.root();
    println!("root {:02X?}", &root);
    root.try_into().expect("unreachable")
}

//=========
// use merkle_patricia_tree::MerklePatriciaTree;
// pub fn mpt_root(proof: Vec<Vec<u8>>) -> [u8; 32] {
//     //   let mut trie = MerklePatriciaTree::new();
//     // trie.update(
//     //   &String::from("doge").into_bytes(),
//     //   &String::from("coin").into_bytes(),
//     // );
//     // let current_root = trie.root;
//     // let doge = trie.get(&String::from("coin").into_bytes());
//     let mut trie = MerklePatriciaTree::new();
//     println!("rolling root {:02X?}", &trie.root);
//     for node in proof {
//         let k = keccak256(&node);
//         trie.update(
//             &k,    // hashed storage key
//             &node, // raw rlp
//         );
//         println!("rolling root {:02X?}", &trie.root);
//     }
//     trie.root.try_into().expect("unreachable")
// }

#[cfg(test)]
mod tests {
    use super::mpt_root;
    use hex_literal::hex;

    #[test]
    fn test_can_verify_eip_1186_storage_proofs() {
        // let key: [u8; 32] =
        //     hex!("8fc241b7eaf929f4c5b3f5bd01abbdc2cc61368ac3c2cca9a28d5d410d4049d5");
        let proof = vec![
        hex!("f90131a0db84880ea6ca86b1065c9a2c61033daff2455d0e3a10867ff300b4863218a18aa07d7afd2ba5ad4c7085699c7505cf9cb67ea074b7116c7b2073f56736498e52d0a0150507169b2f23aa57226a33553af0684d7ee8ebfec67cbe90693640bfe94d19808080a04616444ecc68fd60c58a3705a3dbd7a178af8dbf50e2be26bf9b2e94e89db4a3a026e732b882408cd7b9e39ed706992d0526f0d60193f666181124e807baff6d7fa06512473128eb2f4b680fdcfd7e3d05ec0ad9bdccbfe10dbea0e8519945ce8df780a02cd9a8f9c26e2a581de890b50b387477748c69d7ddcbab84ec280e201ded7b4980a0b92bbcfcacad3b833b4d2a4993069af365b8ae1fb94abe5cd3f89d97ee911462a0f0be3262950058a03bc547c666135e195c9108f123de8111226f5938fbdfae8d808080").to_vec(),
        hex!("f85180808080808080808080a0f86e42085f656503c98a723a490d38856efaca22869239c50173ccca1f402412808080a001a5aff7191fdb70f92336addbc265906d0f57c6c718bed42199aeb2c23a4ae58080").to_vec(),
        hex!("e2a0201a9a6ec067234252fc23d745dd8bcf03e73e895f4374845f3dc65fab5dd47001").to_vec(),
    ];

        let expected_root =
            hex!("9276dd802bae68f79e2c91fe580a53599603818804ede9c7dab86eaae4e97eee");

        let root = mpt_root(proof);

        println!("root {:02X?}", &root);
        println!("xp root {:02X?}", &expected_root);

        assert_eq!(root, expected_root);
    }
}
