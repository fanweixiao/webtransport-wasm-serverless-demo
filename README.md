# WebTransport Serverless Demo

<img alt="image" src="https://user-images.githubusercontent.com/65603/190899824-7c8bb2e4-c26d-4b4a-a3ca-d9a7e4de4d60.png">

1. `rustup target add wasm32-wasi`
1. `cargo build --release --target wasm32-wasi`
1. Get `.env` from http://allegrocloud.io for free
1. `yomo run -r wasmtime target/wasm32-wasi/release/yomo_webassembly_sfn_rust.wasm`
