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

The `.wasm` file produced is around 1.5mb. Which is kinda big!


## How to create a smaller .wasm file

If you'd like to make a smaller .wasm file you need to instruct the compiler to remove all the extra debugging information it adds. Unfortunately you can't do this with stable Rust right now, but nightly Rust has a flag we can use.

To install nightly Rust with WebAssembly support follow the following steps:

1. Install nightly Rust: `rustup install nightly`
2. Make it your default with `rustup default nightly`
3. Install WebAssembly support for nightly `rustup target add wasm32-unknown-unknown`

Now you can run a build a *release* build with all compiler information stripped out:

`cargo rustc --target wasm32-unknown-unknown --release -- -Z strip=symbols`

You must change this line in `index.html` to `fetch('target/wasm32-unknown-unknown/release/hello_triangle_wasm_rust.wasm')` to make sure it points to the release folder.

This new stripped .wasm file is 16kb instead of 1.6mb. 1/100th the size!
