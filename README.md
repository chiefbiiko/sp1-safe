# sp1-safe
🚧 WIP WIP

Prove a Safe multisig over a message in zkzk (w/o doxing the Safe and/or its owners).

Debug `ethereum-trie`:

```
cd ./lib/ethereum-trie
cargo test -- --nocapture
```

Debug the `sp1` program:

```sh
cd ./script

SAFE=0x38Ba7f4278A1482FA0a7bC8B261a9A673336EDDc \
MSG_HASH=0xa225aed0c0283cef82b24485b8b28fb756fc9ce83d25e5cf799d0c8aa20ce6b7 \
RUST_LOG=info \
RUST_BACKTRACE=full \
  cargo run --release
```