# sp1-safe wasm prover

[Rust WASM mini book](https://rustwasm.github.io/docs/book/introduction.html)

```sh
wasm-pack build --release
```

```sh
wasm-pack test --release --firefox --headless
wasm-pack test --release --chrome --headless
wasm-pack test --release --safari --headless
```

Our sp1 fork's branch `wasm-compat` includes small changes to make the prover compile on the `wasm32-unknown-unknown` target, namely replaces `std::time` with [`web-time`](https://github.com/daxpedda/web-time), `std::collections::HashMap` with `hashbrown::HashMap` and primitive `std::` with `core::` imports, fx `marker::PhantomData` etc. We also forced `shard_batch_size()` to `0` and `save_disk_threshold()` to `usize::MAX` to prevent the runtime from saving intermediate shard results to disk (not available in WASM), to that end we also completely commented out `tempfile` imports and usage.