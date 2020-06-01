# Hello, CHIP8!

A CHIP8 emulator written in Rust, meant to be compiled to WebAssembly and run in the browser.

The emulator is hosted on the accompanying [github pages site](https://dtrain157.github.io/chip8/).

## Building from source

The Rust project is set up to use [wasm-pack](https://rustwasm.github.io/wasm-pack/) to build it to wasm. Simply execute `wasm-pack build` in the root directory. This will build the Rust code into WASM in the `pkg` directory.

The web component of the project is set up to use NPM and WebPack. Once the Rust code has been compiled, navigate to the `web` directory and execute `npm install` and `npm run build`.

## Licence

This code is free for you to use under the MIT licence.
