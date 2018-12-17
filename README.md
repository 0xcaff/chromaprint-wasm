# chromaprint-wasm

An WASM wrapper around [rust-chromaprint] built using wasm-bindgen.

## Building

    wasm-pack build --target no-modules

Why did I choose to target `no-modules`? Because every other build target makes
assumptions about the environment which is being used to load the WebAssembly.
These assumptions seemed to be wrong when trying to load the generated npm
package from both parcel and webpack in create-react-app.

## Publishing

    wasm-pack publish

[rust-chromaprint]: https://github.com/0xcaff/rust-chromaprint
