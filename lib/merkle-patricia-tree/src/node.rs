use crate::nibbles;

use rlp::RlpStream;
use sha3::{Digest, Keccak256};

#[derive(Clone, Debug)]
pub enum Node {
    Leaf(Vec<u8>, Vec<u8>),
    Extension(Vec<u8>, Box<Node>),
    Branch([Box<Node>; 16], Option<Vec<u8>>),
    Blank,
}

impl Node {
    pub fn get_encoded(&self, with_check: bool) -> Vec<u8> {
        match &*self {
            Node::Leaf(ref nibbles, ref value) => {
                let mut stream = RlpStream::new_list(2);
                stream.append(&nibbles::nibbles_to_bytes(
                    &nibbles::add_hex_prefix(&nibbles, true)[..],
                ));
                stream.append(value);

                match with_check {
                    true => self._check_rlp_raw_encoding_length(&stream.out()),
                    false => stream.out(),
                }
            }
            Node::Extension(ref nibbles, ref boxed_node) => {
                let mut stream = RlpStream::new_list(2);
                stream.append(&nibbles::nibbles_to_bytes(
                    &nibbles::add_hex_prefix(&nibbles, false)[..],
                ));

                let next_node_rlp = boxed_node.get_encoded(true);

                // if it's has been hashed append normal
                if next_node_rlp.len() == 32 {
                    stream.append(&next_node_rlp);
                } else {
                    stream.append_raw(&next_node_rlp, 1);
                }

                match with_check {
                    true => self._check_rlp_raw_encoding_length(&stream.out()),
                    false => stream.out(),
                }
            }
            Node::Branch(ref boxed_nodes, ref value) => {
                let mut stream = RlpStream::new_list(17);
                for i in 0..16 {
                    let boxed_node_rlp = boxed_nodes[i].get_encoded(true);

                    if boxed_node_rlp.len() == 0 {
                        stream.append_empty_data();
                    } else if boxed_node_rlp.len() == 32 {
                        stream.append(&boxed_node_rlp);
                    } else {
                        stream.append_raw(&boxed_node_rlp, 1);
                    }
                }

                match value {
                    Some(val) => stream.append(val),
                    None => stream.append_empty_data(),
                };

                match with_check {
                    true => self._check_rlp_raw_encoding_length(&stream.out()),
                    false => stream.out(),
                }
            }
            _ => {
                let mut stream = RlpStream::new();
                stream.append_empty_data();
                stream.out()
            }
        }
    }

    fn _check_rlp_raw_encoding_length(&self, rlp_raw: &[u8]) -> Vec<u8> {
        if rlp_raw.len() >= 32 {
            let mut hasher = Keccak256::new();
            hasher.input(rlp_raw);
            return hasher.result().to_vec();
        }

        return rlp_raw.to_vec();
    }
}
