# quad-storage

[![Docs](https://docs.rs/quad-storage/badge.svg?version=0.1.0)](https://docs.rs/quad-storage/0.1.0/quad_url/)
[![Crates.io version](https://img.shields.io/crates/v/quad-storage.svg)](https://crates.io/crates/quad-storage)

This is the crate to save data in persistent local storage in miniquad/macroquad environment. In WASM the data persists even if tab or browser are closed. To achieve that [Web Storage API](https://developer.mozilla.org/en-US/docs/Web/API/Web_Storage_API) is used. To save data on native platforms, data is just saved to the local file named `local.data`.

[Web demo.](https://optozorax.github.io/quad-storage/)

# Code example
```rust
let storage = &mut quad_storage::STORAGE.lock().unwrap();
storage.set("test", "value");
let value = storage.get("test").unwrap();
dbg!(value);
```

# Usage
Add this to your `Cargo.toml` dependencies:
```text
quad-storage = "0.1.0"
```
# Usage in WASM
Add file [`quad-storage/js/quad-storage.js`](https://github.com/optozorax/quad-storage/blob/master/js/quad-storage.js) to your project.

Add file [`sapp-jsutils/js/sapp_jsutils.js`](https://github.com/not-fl3/sapp-jsutils/blob/master/js/sapp_jsutils.js) file to your project.

Add this lines after loading of `gl.js` and before loading of your wasm in your `index.html`:
```html
<script src="sapp_jsutils.js"></script>
<script src="quad-storage.js"></script>
```
Done! Now you can use this crate.