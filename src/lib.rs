#![allow(unused_variables)]

//! # quad-storage
//!
//! This is the crate to save data in persistent local storage in miniquad/macroquad environment. In WASM the data persists even if tab or browser are closed. To achieve that [Web Storage API](https://developer.mozilla.org/en-US/docs/Web/API/Web_Storage_API) is used. To save data on native platforms, data is just saved to the local file named `local.data`.
//!
//! [Web demo.](https://optozorax.github.io/quad-storage/)
//!
//! # Code example
//! ```rust
//! let storage = &mut quad_storage::STORAGE.lock().unwrap();
//! storage.set("test", "value");
//! let value = storage.get("test").unwrap();
//! assert_eq!(value, "value");
//! ```
//!
//! # Usage
//! Add this to your `Cargo.toml` dependencies:
//! ```text
//! quad-storage = "0.1.0"
//! ```
//! # Usage in WASM
//! Add file [`quad-storage/js/quad-storage.js`](https://github.com/optozorax/quad-storage/blob/master/js/quad-storage.js) to your project.
//!
//! Add file [`sapp-jsutils/js/sapp_jsutils.js`](https://github.com/not-fl3/sapp-jsutils/blob/master/js/sapp_jsutils.js) file to your project.
//!
//! Add this lines after loading of `gl.js` and before loading of your wasm in your `index.html`:
//! ```html
//! <script src="sapp_jsutils.js"></script>
//! <script src="quad-storage.js"></script>
//! ```
//! Done! Now you can use this crate.

use lazy_static::lazy_static;
use std::sync::Mutex;

#[cfg(target_arch = "wasm32")]
use quad_storage_sys::*;

#[cfg(not(target_arch = "wasm32"))]
use nanoserde::{DeJson, SerJson};

#[cfg(not(target_arch = "wasm32"))]
use std::collections::HashMap;

/// The local storage with methods to read and write data.
#[cfg_attr(not(target_arch = "wasm32"), derive(DeJson, SerJson))]
pub struct LocalStorage {
    #[cfg(not(target_arch = "wasm32"))]
    local: HashMap<String, String>,
}

#[cfg(target_os = "ios")]
extern crate objc;
#[cfg(target_os = "ios")]
use objc::runtime::Object;
#[cfg(target_os = "ios")]
use objc::runtime::Class;
#[cfg(target_os = "ios")]
use objc::{msg_send, sel, sel_impl};
#[cfg(target_os = "ios")]
use std::ffi::{CStr, c_char};

#[cfg(target_os = "ios")]
fn get_documents_directory() -> String {
    unsafe {
        let ns_file_manager = Class::get("NSFileManager").unwrap();
        let file_manager: *mut Object = msg_send![ns_file_manager, defaultManager];
        let ns_document_directory: u64 = 9; // NSDocumentDirectory
        let urls: *mut Object = msg_send![file_manager, URLsForDirectory:ns_document_directory inDomains:1];
        let url: *mut Object = msg_send![urls, firstObject];
        let path: *const c_char = msg_send![url, fileSystemRepresentation];
        let c_str: &CStr = CStr::from_ptr(path);
        c_str.to_string_lossy().into_owned()
    }
}

#[cfg(all(not(target_arch = "wasm32"), not(target_os = "ios")))]
const LOCAL_FILE: &str = "local.data";

#[cfg(target_os = "ios")]
fn get_local_file_path() -> String {
    get_documents_directory() + "/local.data"
}

#[cfg(all(not(target_arch = "wasm32"), not(target_os = "ios")))]
fn get_local_file_path() -> &'static str {
    LOCAL_FILE
}

impl Default for LocalStorage {
    fn default() -> Self {
        #[cfg(target_arch = "wasm32")]
        {
            Self {}
        }
        #[cfg(not(target_arch = "wasm32"))]
        {
            if let Ok(file) = std::fs::read_to_string(get_local_file_path()) {
                LocalStorage::deserialize_json(&file).unwrap()
            } else {
                LocalStorage {
                    local: Default::default(),
                }
            }
        }
    }
}

impl LocalStorage {
    #[allow(clippy::len_without_is_empty)]
    pub fn len(&self) -> usize {
        #[cfg(target_arch = "wasm32")]
        {
            len()
        }
        #[cfg(not(target_arch = "wasm32"))]
        {
            self.local.len()
        }
    }

    /// Get key by its position
    pub fn key(&self, pos: usize) -> Option<String> {
        #[cfg(target_arch = "wasm32")]
        {
            key(pos)
        }
        #[cfg(not(target_arch = "wasm32"))]
        {
            self.local.keys().nth(pos).cloned()
        }
    }

    pub fn get(&self, key: &str) -> Option<String> {
        #[cfg(target_arch = "wasm32")]
        {
            get(key)
        }
        #[cfg(not(target_arch = "wasm32"))]
        {
            self.local.get(key).cloned()
        }
    }
    pub fn set(&mut self, key: &str, value: &str) {
        #[cfg(target_arch = "wasm32")]
        {
            set(key, value)
        }
        #[cfg(not(target_arch = "wasm32"))]
        {
            self.local.insert(key.to_string(), value.to_string());
            self.save();
        }
    }
    pub fn remove(&mut self, key: &str) {
        #[cfg(target_arch = "wasm32")]
        {
            remove(key)
        }
        #[cfg(not(target_arch = "wasm32"))]
        {
            self.local.remove(key);
            self.save();
        }
    }
    pub fn clear(&mut self) {
        #[cfg(target_arch = "wasm32")]
        {
            clear()
        }
        #[cfg(not(target_arch = "wasm32"))]
        {
            self.local.clear();
            self.save();
        }
    }

    #[cfg(not(target_arch = "wasm32"))]
    fn save(&self) {
        std::fs::write(get_local_file_path(), self.serialize_json()).unwrap();
    }
}

lazy_static! {
    /// Global singleton to access [`LocalStorage`].
    ///
    /// Usage:
    /// ```rust
    /// let storage = &mut quad_storage::STORAGE.lock().unwrap();
    /// ```
    pub static ref STORAGE: Mutex<LocalStorage> = Mutex::new(Default::default());
}
