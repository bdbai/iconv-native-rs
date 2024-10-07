# iconv-native-rs

[![crates.io](https://img.shields.io/crates/v/iconv-native.svg)](https://crates.io/crates/iconv-native)
[![Released API docs](https://docs.rs/iconv-native/badge.svg)](https://docs.rs/iconv-native)
[![MIT licensed](https://img.shields.io/badge/license-MIT-blue.svg)](./LICENSE)
[![CI](https://github.com/bdbai/iconv-native-rs/actions/workflows/run-tests.yaml/badge.svg)](https://github.com/bdbai/iconv-native-rs/actions/workflows/run-tests.yaml)

A lightweight text encoding converter based on platform native APIs or [libiconv].

## Platforms

### Windows

By default this crate uses [`MultiByteToWideChar`](https://learn.microsoft.com/en-us/windows/win32/api/stringapiset/nf-stringapiset-multibytetowidechar) and [`WideCharToMultiByte`](https://learn.microsoft.com/en-us/windows/win32/api/stringapiset/nf-stringapiset-widechartomultibyte) functions, controlled by feature `win32`. Since UTF-32 is not supported by these functions, [`widestring`](https://docs.rs/widestring) crate is used to convert UTF-32 to UTF-16 and vice versa.

You may also disable disable default features and enable `libiconv` to use the [libiconv] library instead.

### Linux

On Linux with glibc, the built-in [`iconv`](https://man7.org/linux/man-pages/man3/iconv.3.html) is used by default, controlled by feature `libc-iconv`. You may also disable default features and enable `libiconv` to use the [libiconv] library instead.

Other libcs may not have an `iconv` implementation that is compatible with glibc's (specifically the `//IGNORE` and `//TRANSLIT` extensions and proper BOM handling), hence `libc-iconv` feature does not apply to them. By default, `fallback-libiconv` feature applies and will link to the [libiconv] library. Make sure to have [libiconv] installed on user's system.

### macOS

Same as Linux with glibc.  You may also disable default features and enable `libiconv` to use the [libiconv] library instead.

### Web (WASM)

Uses [`TextDecoder`] and [`TextEncoder`] Web APIs. [`widestring`](https://docs.rs/widestring/latest/widestring/) crate is used to handle UTF-16 and UTF-32 related conversions.

> [!IMPORTANT]
> As per [Encoding Standard](https://encoding.spec.whatwg.org/#interface-textencoder), a standard-compliant browser supports only UTF-8 when using [`TextEncoder`], hence conversions **to** any encodings other than UTF-8/UTF-16/UTF-32 (including LE/BE variants) are not supported and will result in an `UnknownConversion` error.
> Consider import a [polyfill](https://www.npmjs.com/package/text-encoding-polyfill) and enable `wasm-nonstandard-allow-legacy-encoding` feature if full encoding support is required, in which case 
> most of the encodings will work. However, there is no guarantee as it is not a standard-compliant behavior.
>
> Conversions **from** legacy encodings are not affected by this limitation. See [Encoding Standard] for more details.

[`TextDecoder`]: https://developer.mozilla.org/en-US/docs/Web/API/TextDecoder
[`TextEncoder`]: https://developer.mozilla.org/en-US/docs/Web/API/TextEncoder
[Encoding Standard]: https://encoding.spec.whatwg.org/#interface-textencoder

### Other

On other platforms, the [libiconv] library is used by default, controlled by feature `fallback-libiconv`.

[libiconv]: https://www.gnu.org/software/libiconv/
