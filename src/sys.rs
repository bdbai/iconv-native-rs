use cfg_if::cfg_if;

#[cfg(all(feature = "libc-iconv", feature = "libiconv"))]
compile_error!("`libc-iconv` and `libiconv` features are mutually exclusive");

cfg_if! {
    if #[cfg(all(windows, feature = "win32"))] {
        mod win;
        use win as inner;
    } else if #[cfg(all(target_arch = "wasm32", feature = "web-encoding"))] {
        mod wasm;
        use wasm as inner;
    } else if #[cfg(all(
        feature = "libc-iconv",
        any(
            all(target_env = "gnu", target_os = "linux"),
            target_os = "hurd",
            target_vendor = "apple"
        )
    ))] {
        mod iconv;
        use iconv as inner;
    } else if #[cfg(any(feature = "libiconv", feature = "fallback-libiconv"))] {
        mod iconv;
        use iconv as inner;
    }
}

pub(crate) use inner::{convert_lossy, decode_lossy};
