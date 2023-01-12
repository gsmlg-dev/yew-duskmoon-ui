# yew-app-sample
Sample yew app

## Install dependencies

```
rustup target add wasm32-unknown-unknown
cargo install wasm-pack
cargo install trunk 
cargo install wasm-bindgen-cli
```


## Run

```
# dev
trunk serve

# build
trunk build --release

# test
wasm-pack test --headless --chrome
```

