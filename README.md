# iconv-native-rs

A lightweight text encoding converter based on platform native API or libiconv.

// TODO: badges
// TODO: doc

## Platforms

### Windows

By default this crate uses [`MultiByteToWideChar`](https://learn.microsoft.com/en-us/windows/win32/api/stringapiset/nf-stringapiset-multibytetowidechar) and [`WideCharToMultiByte`](https://learn.microsoft.com/en-us/windows/win32/api/stringapiset/nf-stringapiset-widechartomultibyte) functions, controlled by feature `win32`. Since UTF-32 is not supported by these functions, [`widestring`](https://docs.rs/widestring) crate is used to convert UTF-32 to UTF-16 and vice versa.

You may also disable disable default features and enable `libiconv` to use the `libiconv` library instead.

### Unix (WIP)

TODO: glibc iconv or libiconv

### Web (WASM) (WIP)

Uses [`TextDecoder`] and [`TextEncoder`] Web APIs. [`widestring`](https://docs.rs/widestring/latest/widestring/) crate is used to handle UTF-16 and UTF-32 related conversions.

> [!IMPORTANT]
> As per [Encoding Standard](https://encoding.spec.whatwg.org/#interface-textencoder), a standard-compliant browser supports only UTF-8 when using [`TextEncoder`], hence conversions to any encodings other than UTF-8/UTF-16/UTF-32 (including LE/BE variants) are not supported and will result in an `UnknownToEncoding` error.
> Consider import a [polyfill](https://www.npmjs.com/package/text-encoding-polyfill) and enable `wasm_nonstandard_allow_legacy_encoding` feature if full encoding support is required, in which case 
> most of the encodings will work. However, there is no guarantee as it is not a standard-compliant behavior.

[`TextDecoder`]: https://developer.mozilla.org/en-US/docs/Web/API/TextDecoder
[`TextEncoder`]: https://developer.mozilla.org/en-US/docs/Web/API/TextEncoder
