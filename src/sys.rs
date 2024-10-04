#[cfg(all(windows, feature = "win32"))]
mod win;
#[cfg(all(windows, feature = "win32"))]
use win as inner;

#[cfg(all(target_arch = "wasm32", feature = "web_encoding"))]
mod wasm;
#[cfg(all(target_arch = "wasm32", feature = "web_encoding"))]
use wasm as inner;

pub(crate) use inner::{convert_lossy, decode_lossy};
