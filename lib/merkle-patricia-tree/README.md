# :herb: merkle-patricia-tree

Simple implementation of [Patricia Merkle Tree](https://github.com/ethereum/wiki/wiki/Patricia-Tree). All of the merkle tries in Ethereum use a Merkle Patricia Trie.

> The modified Merkle Patricia tree (trie) provides a persistent data structure to map between arbitrary-length binary data (byte arrays). It is defined in terms of a mutable data structure to map between 256-bit binary fragments and arbitrary-length binary data. The core of the trie, and its sole requirement in terms of the protocol specification is to provide a single 32-byte value that identifies a given set of key-value pairs.

&nbsp;

***

&nbsp;

## :page_with_curl: How to use it

```rust
let mut trie = MerklePatriciaTree::new();

trie.update(
  &String::from("doge").into_bytes(),
  &String::from("coin").into_bytes(),
);

let current_root = trie.root;

let doge = trie.get(&String::from("coin").into_bytes());
```

&nbsp;

***

&nbsp;

## :checkered_flag: Test

### How to run

```
cargo test
```

### Results

```
running 11 tests
test nibbles::tests::should_add_hex_prefix_with_even_nibbles_and_with_terminator ... ok
test nibbles::tests::should_add_hex_prefix_with_even_nibbles_and_without_terminator ... ok
test nibbles::tests::should_add_hex_prefix_with_odd_nibbles_and_with_terminator ... ok
test nibbles::tests::should_add_hex_prefix_with_odd_nibbles_and_without_terminator ... ok
test nibbles::tests::should_calculate_the_prefix_length_between_2_array_of_nibbles ... ok
test nibbles::tests::should_convert_bytes_into_nibbles ... ok
test nibbles::tests::should_convert_nibbles_into_bytes ... ok
test trie::tests::should_be_an_empty_trie ... ok
test trie::tests::should_store_and_get_the_value ... ok
test trie::tests::should_store_update_and_get_the_value ... ok
test trie::tests::should_calculate_the_correct_root_hash_and_get_a_value ... ok

test result: ok. 11 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

&nbsp;

***

&nbsp;


## :clipboard: TODO

- [ ] Verify a proof
- [ ] Delete a node given a key