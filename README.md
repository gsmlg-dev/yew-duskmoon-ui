# **Yew Duskmoon UI**

Duskmoon UI Component Library.

This package use `stylist` to embbed css in components, so no extra CSS file is needed.

# Documents and Demo

[Link](https://gsmlg-dev.github.io/yew-duskmoon-ui/)

# Development

## Install dependencies

```
rustup target add wasm32-unknown-unknown
cargo install wasm-pack
cargo install trunk 
cargo install wasm-bindgen-cli
```

## Run

```
cd example

# dev
trunk serve

# build
trunk build --release

```
