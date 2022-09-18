# WebTransport Serverless Demo

1. `rustup target add wasm32-wasi`
1. `cargo build --release --target wasm32-wasi`
1. Get `.env` from http://allegrocloud.io for free
1. `yomo run -r wasmtime target/wasm32-wasi/release/yomo_webassembly_sfn_rust.wasm`
