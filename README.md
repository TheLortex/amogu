## A. Mongus

### ???

This is a client-side, Webassembly-based filter to hide amongus characters in your images.

Example:

- Original:

- Processed with size=1, count=1000

### Use it yourself

This page: 

### Development

This uses Rust + Webassembly, and `npm` for the Js side.

For the Rust side, use `wasm-pack build` in the root folder. This generates a `pkg` folder with all the Js / WASM goodness.

For the Web side, use `npm run start`, that uses webpack to bundle everything.

