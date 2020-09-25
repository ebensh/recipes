# Recipes

This is a repository for displaying recipes using Rust and Yew.

## Tools

- Language: [Rust](https://www.rust-lang.org/learn)
- Web framework: [Yew](https://yew.rs/)
- Web library: web-sys [context](https://yew.rs/docs/en/getting-started/choose-web-library/)
- Wasm Build Tool: [wasm-pack](https://yew.rs/docs/en/getting-started/project-setup/#wasm-packhttpsrustwasmgithubiodocswasm-pack)
  - `cargo install wasm-pack`

## Useful References

- https://dev.to/davidedelpapa/yew-tutorial-01-introduction-13ce

## Setup

```bash
# Yew setup - install wasm-pack
cargo install wasm-pack

# Create the Rust project as a **library**
cargo new --lib yew-app && cd yew-app

# Add `yew` and `wasm-bindgen` to dependencies.
# See https://yew.rs/docs/en/getting-started/build-a-sample-app/
# for details.

# Use wasm-pack to build the website
wasm-pack build --target web --out-name wasm --out-dir ./static

# And serve it :)
cargo +nightly install miniserve
miniserve ./static --index index.html
```