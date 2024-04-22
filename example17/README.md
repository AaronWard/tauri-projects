# Tauri + Vue 3

This template should help get you started developing with Tauri + Vue 3 in Vite. The template uses Vue 3 `<script setup>` SFCs, check out the [script setup docs](https://v3.vuejs.org/api/sfc-script-setup.html#sfc-script-setup) to learn more.

## Recommended IDE Setup

- [VS Code](https://code.visualstudio.com/) + [Volar](https://marketplace.visualstudio.com/items?itemName=Vue.volar) + [Tauri](https://marketplace.visualstudio.com/items?itemName=tauri-apps.tauri-vscode) + [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer)

---


Commands to get wasm of `/wasm/` folder.

```bash
    9  cd example17/
  490  ls
  491  cd wasm/
  492  cargo new --lib rust_wasm
  493  cd rust_wasm
  494  cargo build --target wasm32-unknown-unknown
  495  wasm-bindgen target/wasm32-unknown-unknown/debug/rust_wasm.wasm --out-dir ./pkg --web
  496  pwd
  497  rustup target add wasm32-unknown-unknown
  498  cargo build --target wasm32-unknown-unknown
  499  rustc --explain E0463
  500  rustup target add wasm32-unknown-unknown
  501  cd example17/
  502  cd wasm/rust_wasm/
  503  ls
  504  cargo install wasm-pack
  505  wasm-pack build --target web
  506  rustup default nightly
  507  cargo +nightly build --target wasm32-unknown-unknown -Z build-std
  508  cargo build --target wasm32-unknown-unknown
  509  rustup show
  510  rustup default nightly
  511  rustup target add wasm32-unknown-unknown
  512  cargo +nightly build --target wasm32-unknown-unknown -Z build-std
  513  cargo build --target wasm32-unknown-unknown
  514  cargo build --target wasm32-unknown-unknown -Z build-std
  515  cargo build --target wasm32-unknown-unknown
  516  wasm-pack build --target web
  517  echo $PATH
  518  # Ensure the path to rustup is included, and it should precede other paths to rust tools.
  519   source $HOME/.cargo/env
  520  cargo build --target wasm32-unknown-unknown
  521  rustup default nightly
  522  rustup target add wasm32-unknown-unknown
  523  cargo build --target wasm32-unknown-unknown
  524  cargo build --target wasm32-unknown-unknown
  525  export PATH="$HOME/.cargo/bin:$PATH"
  526  wasm-pack build --target web
  527  wasm-pack build --target web
  528  history
(base) LAMU0CLP74YXVX6:rust_wasm award40$ 
```

NOTE!!!!: you need to move out of `pkg` because the host application can't see it - the `.wasm` and `.js` file are moved to `wasm/out`

<!-- wasm-bindgen --out-dir out --target web wasm/out/rust_wasm_bg.wasm -->