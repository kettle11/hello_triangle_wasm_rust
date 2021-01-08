# Hello Triangle with WebGL and Rust

This is a minimal example that demonstrates how to render a triangle with WebAssembly, WebGL, Rust, and zero dependencies.

To build the project perform the following steps:

1. Install Rust.
2. Install the `wasm32-unknown-unknown` target: `rustup target add wasm32-unknown-unknown`
3. Build the directory: `cargo build --target wasm32-unknown-unknown`
4. Run a local server to host the project directory. `devserver` is the easiest to setup:
    * First run `cargo install devserver`
    * Then simply run `devserver`. 
5. Navigate to `localhost:8080` and view the triangle!
