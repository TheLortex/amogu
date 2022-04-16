# A. Mongus

Go to: http://www.lortex.org/amogu/

## ???

This is a client-side, Webassembly-based filter to hide amongus characters in your images.

Example:

- Original:

<img src="https://github.com/TheLortex/amogu/raw/main/.screenshots/mongus.jpg" width="400" />

- Processed with size=2, count=3000, contrast=5:

<img src="https://github.com/TheLortex/amogu/raw/main/.screenshots/mongus_processed.png" width="400" />

- See what the air is made of:

<img src="https://github.com/TheLortex/amogu/raw/main/.screenshots/mongus_processed_zoom.png" width="400" />

## Development

This uses Rust + Webassembly, and `npm` for the Js side.

For the Rust side, use `wasm-pack build` in the root folder. This generates a `pkg` folder with all the Js / WASM goodness.

For the Web side, use `npm run start`, that uses webpack to bundle everything.

### Issues / feature requests

I'd gladly accept contributions but I won't spend a lot of time improving the thing.
