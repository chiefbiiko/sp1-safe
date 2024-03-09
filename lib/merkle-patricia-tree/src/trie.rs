use std::collections::HashMap;

use crate::nibbles;
use crate::node::Node;
use sha3::{Digest, Keccak256};
use std::str;

static KECCAK256_RLP: &str = "KECCAK256_RLP_S";

fn empty_branch_nodes() -> [Box<Node>; 16] {
    [
        Box::new(Node::Blank),
        Box::new(Node::Blank),
        Box::new(Node::Blank),
        Box::new(Node::Blank),
        Box::new(Node::Blank),
        Box::new(Node::Blank),
        Box::new(Node::Blank),
        Box::new(Node::Blank),
        Box::new(Node::Blank),
        Box::new(Node::Blank),
        Box::new(Node::Blank),
        Box::new(Node::Blank),
        Box::new(Node::Blank),
        Box::new(Node::Blank),
        Box::new(Node::Blank),
        Box::new(Node::Blank),
    ]
}

pub struct MerklePatriciaTree {
    pub root: Vec<u8>,
    root_node: Node,
    db: HashMap<Vec<u8>, Vec<u8>>,
}

impl MerklePatriciaTree {
    pub fn new() -> MerklePatriciaTree {
        MerklePatriciaTree {
            root: KECCAK256_RLP.as_bytes().to_vec(),
            db: HashMap::new(),
            root_node: Node::Blank,
        }
    }

    pub fn update(&mut self, key: &[u8], value: &[u8]) {
        self.root_node = self._maybe_update_and_delete_storage(
            &self.root_node.clone(),
            &nibbles::bytes_to_nibbles(&key),
            &value,
        );

        let mut hasher = Keccak256::new();
        hasher.input(self.root_node.get_encoded(false));
        self.root = hasher.result().to_vec();
    }

    fn _maybe_update_and_delete_storage(&mut self, node: &Node, key: &[u8], value: &[u8]) -> Node {
        let new_node = self._update(&node, &key, &value);

        // true -> calculate the hash on the rlp encoded value
        let new_hash_key = new_node.get_encoded(true);
        let old_hash_key = node.get_encoded(true);
        // false -> get rlp encoded value
        let new_value = new_node.get_encoded(false);

        if new_hash_key != old_hash_key {
            if new_value.len() >= 32 {
                self.db.insert(new_hash_key, new_value);
                self.db.remove(&old_hash_key);
            }
        }

        new_node
    }

    fn _update(&mut self, node: &Node, nibbles: &[u8], value: &[u8]) -> Node {
        match node {
            Node::Blank => Node::Leaf(nibbles.to_vec(), value.to_vec()),
            Node::Leaf(ref node_nibbles, ref node_value) => {
                let current_nibbles = node_nibbles;
                let prefix = nibbles::calculate_prefix_length(&current_nibbles.clone(), &nibbles);

                let remaining_current_nibbles = &current_nibbles[prefix..current_nibbles.len()];
                let remaining_new_nibbles = &nibbles[prefix..nibbles.len()];

                let new_node;

                if remaining_current_nibbles.len() == 0 && remaining_new_nibbles.len() == 0 {
                    return Node::Leaf(node_nibbles.to_vec(), value.to_vec());
                }
                // current key exhausted. Create a branch node...
                else if remaining_current_nibbles.len() == 0 {
                    let new_leaf_node = Node::Leaf(
                        remaining_new_nibbles[1..remaining_new_nibbles.len()].to_vec(),
                        value.to_vec(),
                    );

                    // branches used for links among nodes
                    let mut branches_nodes = empty_branch_nodes();
                    branches_nodes[remaining_new_nibbles[0] as usize] = Box::new(new_leaf_node);

                    new_node = Node::Branch(branches_nodes, Some(node_value.to_vec()));
                } else {
                    let mut branches_nodes = empty_branch_nodes();

                    // since it's a leaf node, there are no more child
                    branches_nodes[remaining_current_nibbles[0] as usize] = Box::new(Node::Leaf(
                        remaining_current_nibbles[1..remaining_current_nibbles.len()].to_vec(),
                        node_value.clone(),
                    ));

                    // updating value. if key is of len equal to 0, update the branch value,
                    // otherwise create a new branch with a leaf node
                    if remaining_new_nibbles.len() == 0 {
                        new_node = Node::Branch(branches_nodes, Some(value.to_vec()));
                    } else {
                        branches_nodes[remaining_new_nibbles[0] as usize] = Box::new(Node::Leaf(
                            remaining_new_nibbles[1..remaining_new_nibbles.len()].to_vec(),
                            value.to_vec(),
                        ));
                        new_node = Node::Branch(branches_nodes, None);
                    }
                }

                return self._maybe_wrap_into_an_extension(prefix, &current_nibbles, &new_node);
            }
            Node::Extension(ref node_nibbles, ref boxed_node) => {
                let current_nibbles = node_nibbles;
                let prefix = nibbles::calculate_prefix_length(&current_nibbles.clone(), &nibbles);

                let remaining_current_nibbles = &current_nibbles[prefix..current_nibbles.len()];
                let remaining_new_nibbles = &nibbles[prefix..nibbles.len()];

                let new_node;

                if remaining_current_nibbles.len() == 0 && remaining_new_nibbles.len() == 0 {
                    new_node = self._maybe_update_and_delete_storage(
                        boxed_node,
                        remaining_new_nibbles,
                        value,
                    );
                } else if remaining_current_nibbles.len() == 0 {
                    new_node = self._maybe_update_and_delete_storage(
                        boxed_node,
                        remaining_new_nibbles,
                        value,
                    );
                } else {
                    let mut branches_nodes = empty_branch_nodes();

                    if remaining_current_nibbles.len() == 1 {
                        // boxed_node is a leaf
                        branches_nodes[remaining_current_nibbles[0] as usize] = boxed_node.clone();
                    } else {
                        // updating next node. since it's an extension it is impossible that the
                        // next one is an extension again
                        if let Node::Leaf(ref _next_nibbles, ref next_value) = **boxed_node {
                            let next_leaf_node = Node::Leaf(
                                remaining_current_nibbles[1..remaining_current_nibbles.len()]
                                    .to_vec(),
                                next_value.clone(),
                            );

                            branches_nodes[remaining_current_nibbles[0] as usize] =
                                Box::new(next_leaf_node);
                        } else if let Node::Branch(ref next_boxed_nodes, ref next_value) =
                            **boxed_node
                        {
                            // if it's a branch node, there should be an extensio node between
                            let next_branch_node =
                                Node::Branch(next_boxed_nodes.clone(), next_value.clone());

                            let next_extension_node = Node::Extension(
                                remaining_current_nibbles[1..remaining_current_nibbles.len()]
                                    .to_vec(),
                                Box::new(next_branch_node),
                            );

                            branches_nodes[remaining_current_nibbles[0] as usize] =
                                Box::new(next_extension_node);
                        } else if let Node::Blank = **boxed_node {
                            branches_nodes[remaining_current_nibbles[0] as usize] =
                                Box::new(Node::Blank);
                        }
                    }

                    // updating value. if key is of len equal to 0, update the branch value,
                    // otherwise create a new branch with a leaf node as a child
                    if remaining_new_nibbles.len() == 0 {
                        new_node = Node::Branch(branches_nodes, Some(value.to_vec()));
                    } else {
                        branches_nodes[remaining_new_nibbles[0] as usize] = Box::new(Node::Leaf(
                            remaining_new_nibbles[1..remaining_new_nibbles.len()].to_vec(),
                            value.to_vec(),
                        ));
                        new_node = Node::Branch(branches_nodes, None);
                    }
                }

                return self._maybe_wrap_into_an_extension(prefix, &current_nibbles, &new_node);
            }
            Node::Branch(ref boxed_nodes, ref node_value) => {
                // reached end -> update branch value
                if nibbles.len() == 0 {
                    return Node::Branch(boxed_nodes.clone(), Some(value.to_vec()));
                }

                let new_node = &self._maybe_update_and_delete_storage(
                    &boxed_nodes[nibbles[0] as usize],
                    &nibbles[1..nibbles.len()],
                    &value,
                );

                let mut updated_boxed_branches = boxed_nodes.clone();
                updated_boxed_branches[nibbles[0] as usize] = Box::new(new_node.clone());

                return Node::Branch(updated_boxed_branches, node_value.clone());
            }
        }
    }

    fn _maybe_wrap_into_an_extension(&self, prefix: usize, nibbles: &[u8], node: &Node) -> Node {
        if prefix > 0 {
            return Node::Extension(nibbles[0..prefix].to_vec(), Box::new(node.clone()));
        } else {
            return node.clone();
        }
    }

    pub fn get(&self, key: &[u8]) -> Result<Vec<u8>, &'static str> {
        let found_node = self._get(&self.root_node.clone(), &nibbles::bytes_to_nibbles(&key));
        match found_node {
            Node::Leaf(ref _nibbles, ref value) => Ok(value.clone()),
            Node::Branch(ref _boxed_nodes, ref node_value) => match node_value {
                Some(value) => Ok(value.clone()),
                None => Err("Empty Node"),
            },
            _ => Err("Not Found"),
        }
    }

    fn _get(&self, node: &Node, nibbles: &[u8]) -> Node {
        match node {
            Node::Blank => Node::Blank,
            Node::Leaf(ref node_nibbles, ref _value) => {
                if nibbles.to_vec() == *node_nibbles {
                    return node.clone();
                }
                Node::Blank
            }
            Node::Extension(ref node_nibbles, ref boxed_node) => {
                let prefix = nibbles::calculate_prefix_length(&node_nibbles.clone(), &nibbles);

                if prefix == node_nibbles.len() {
                    return self._get(&boxed_node, &nibbles[prefix..nibbles.len()]);
                }
                Node::Blank
            }
            Node::Branch(ref boxed_nodes, ref _node_value) => {
                if nibbles.len() == 0 {
                    return node.clone();
                }
                return self._get(
                    &boxed_nodes[nibbles[0] as usize],
                    &nibbles[1..nibbles.len()],
                );
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_be_an_empty_trie() {
        let trie = MerklePatriciaTree::new();
        assert!(trie.root == KECCAK256_RLP.as_bytes().to_vec());
        assert!(trie.db.is_empty());
    }

    #[test]
    fn should_store_and_get_the_value() {
        let mut trie = MerklePatriciaTree::new();
        trie.update(
            &String::from("hello").into_bytes(),
            &String::from("world").into_bytes(),
        );

        let result = trie.get(&String::from("hello").into_bytes()).unwrap();
        assert!(result == String::from("world").into_bytes());
    }

    #[test]
    fn should_store_update_and_get_the_value() {
        let mut trie = MerklePatriciaTree::new();
        trie.update(
            &String::from("hello").into_bytes(),
            &String::from("world").into_bytes(),
        );

        trie.update(
            &String::from("hello").into_bytes(),
            &String::from("world2").into_bytes(),
        );

        let result = trie.get(&String::from("hello").into_bytes()).unwrap();
        assert!(result == String::from("world2").into_bytes());
    }

    #[test]
    fn should_not_get_anything_because_of_not_existing_key() {
        let mut trie = MerklePatriciaTree::new();
        trie.update(
            &String::from("hello").into_bytes(),
            &String::from("world").into_bytes(),
        );

        let result = trie.get(&String::from("byebye").into_bytes());
        if let Err(error) = result {
            assert!(error == String::from("Not Found"));
        }
    }

    #[test]
    fn should_calculate_the_correct_root_hash_and_get_a_value() {
        // taken from here: https://github.com/ethereum/wiki/wiki/Patricia-Tree#example-trie
        let mut trie = MerklePatriciaTree::new();
        trie.update(
            &String::from("do").into_bytes(),
            &String::from("verb").into_bytes(),
        );

        trie.update(
            &String::from("dog").into_bytes(),
            &String::from("puppy").into_bytes(),
        );

        trie.update(
            &String::from("doge").into_bytes(),
            &String::from("coin").into_bytes(),
        );

        trie.update(
            &String::from("horse").into_bytes(),
            &String::from("stallion").into_bytes(),
        );

        assert!(
            trie.root
                == vec![
                    0x59, 0x91, 0xbb, 0x8c, 0x65, 0x14, 0x14, 0x8a, 0x29, 0xdb, 0x67, 0x6a, 0x14,
                    0xac, 0x50, 0x6c, 0xd2, 0xcd, 0x57, 0x75, 0xac, 0xe6, 0x3c, 0x30, 0xa4, 0xfe,
                    0x45, 0x77, 0x15, 0xe9, 0xac, 0x84
                ]
        );

        let result = trie.get(&String::from("horse").into_bytes()).unwrap();
        assert!(result == String::from("stallion").into_bytes());
    }
}
