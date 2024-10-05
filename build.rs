use std::env::var;

fn main() {
    let feature_libiconv = var("CARGO_FEATURE_LIBICONV").is_ok();
    let feature_fallback_libiconv = var("CARGO_FEATURE_FALLBACK_LIBICONV").is_ok();
    let is_windows = var("CARGO_CFG_WINDOWS").is_ok();
    let is_using_win32 = var("CARGO_FEATURE_WIN32").is_ok() && is_windows;
    let is_using_web_encoding = var("CARGO_FEATURE_WEB_ENCODING").is_ok()
        && var("CARGO_CFG_TARGET_ARCH").as_deref() == Ok("wasm32");
    let is_linux = var("CARGO_CFG_TARGET_OS").as_deref() == Ok("linux");
    let is_gnu = var("CARGO_CFG_TARGET_ENV").as_deref() == Ok("gnu");
    let is_hurd = var("CARGO_CFG_TARGET_OS").as_deref() == Ok("hurd");
    let is_apple = var("CARGO_CFG_TARGET_VENDOR").as_deref() == Ok("apple");
    let is_using_libc_iconv =
        var("CARGO_FEATURE_LIBC_ICONV").is_ok() && ((is_gnu && is_linux) || is_hurd || is_apple);
    let is_using_fallback = !(is_using_libc_iconv || is_using_win32 || is_using_web_encoding);
    if feature_libiconv || (feature_fallback_libiconv && is_using_fallback) {
        if is_windows {
            #[cfg(target_env = "msvc")]
            vcpkg::find_package("libiconv").unwrap();
        } else {
            println!("cargo:rustc-link-lib=iconv");
        }
    }
}
