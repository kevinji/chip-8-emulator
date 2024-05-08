# CHIP-8 emulator
A CHIP-8 emulator written in Rust that compiles to WebAssembly using [`wasm-bindgen`](https://github.com/rustwasm/wasm-bindgen) and [`wasm-pack`](https://github.com/rustwasm/wasm-pack). **Read my [blog post](https://kevinji.com/posts/chip8) about the project!**

No JavaScript other than the Wasm module instantiation has been manually written. Instead, bindings to JavaScript APIs has been done automatically using the [`js-sys`](https://crates.io/crates/js-sys) and [`web-sys`](https://crates.io/crates/web-sys) crates.

## Instructions
```bash
cargo build && ./build-wasm.sh
cargo run -p chip-8-server
```

Then, browse to [http://127.0.0.1:3000](http://127.0.0.1:3000/).

## Helpful resources
- [Cowgod's Chip-8 Technical Reference](http://devernay.free.fr/hacks/chip8/C8TECH10.HTM)
- [How to write an emulator (CHIP-8 interpreter)](https://web.archive.org/web/20230411151659/http://www.multigesture.net/articles/how-to-write-an-emulator-chip-8-interpreter/)
- [Guide to making a CHIP-8 emulator](https://tobiasvl.github.io/blog/write-a-chip-8-emulator/)
