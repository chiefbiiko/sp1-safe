# sp1-safe

Prove a Safe multisig over a message in zkzk (w/o doxing the Safe and/or its owners).

Build the RISC-V ELF binary:

```sh
cd ./program
cargo prove build
```

---

Run the `sp1` program:

```sh
cd ./script

SAFE=0x38Ba7...336EDDc \
MSG_HASH=0xa225aed0c0283cef82b24485b8b28fb756fc9ce83d25e5cf799d0c8aa20ce6b7 \
RUST_LOG=info \
RUST_BACKTRACE=full \
  time cargo run --release
```

---

Run the server:

```sh
cargo build --manifest-path ./server/Cargo.toml --release
RUST_LOG=info ./server/target/release/sp1-safe-server
```