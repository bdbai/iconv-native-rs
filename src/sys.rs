#[cfg(all(windows, feature = "win32"))]
mod win;
#[cfg(all(windows, feature = "win32"))]
use win as inner;

pub(crate) use inner::{convert_lossy, decode_lossy};
