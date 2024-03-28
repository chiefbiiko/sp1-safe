# sp1-safe

Prove a Safe multisig over a message in zk.

---

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

Build and run the server:

```sh
cargo build --manifest-path ./server/Cargo.toml --release
RUST_LOG=info ./server/target/release/sp1-safe-server
```

---

Fetch a prebuilt `sp1-safe-server` binary and run it as a systemd service:
<!-- https://0pointer.net/blog/dynamic-users-with-systemd.html -->
```sh
case "$(uname -a)" in
  Linux*)  os=linux  ;;
  Darwin*) os=darwin ;;
esac
curl -sSfL https://TODO-$os.gz | gunzip > /usr/local/bin/sp1-safe-server
curl -sSfL https://raw.githubusercontent.com/chiefbiiko/sp1-safe/main/server/sp1-safe-server.service | sudo tee /etc/systemd/system/sp1-safe-server.service
systemctl daemon-reload
systemctl start sp1-safe-server.service
```