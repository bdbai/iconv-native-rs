A lightweight text encoding converter based on platform native APIs or [libiconv].

## Usage

Use [`convert`] or [`convert_lossy`] to convert text between encodings.

```rust
use iconv_native::convert;

let output = convert(b"\x82\xb3\x83\x86\x82\xe8", "Shift_JIS", "UTF-16LE")?;
assert_eq!(output, b"\x55\x30\xe6\x30\x8a\x30");
# Ok::<(), iconv_native::ConvertError>(())
```

If target encoding is UTF-8 and the output is going to be treated as a Rust-native [`String`], use [`decode`] or [`decode_lossy`] instead.

```rust
use iconv_native::decode;

let output = decode(b"\xa4\xaa\xa4\xe4\xa4\xb9\xa4\xdf", "GB18030")?;
assert_eq!(output, "おやすみ");
# Ok::<(), iconv_native::ConvertError>(())
```

There are some minor differences between these functions specifically for BOM handling. See the documentation of each function for more details.

## Platforms

### Windows

By default this crate uses [`MultiByteToWideChar`](https://learn.microsoft.com/en-us/windows/win32/api/stringapiset/nf-stringapiset-multibytetowidechar) and [`WideCharToMultiByte`](https://learn.microsoft.com/en-us/windows/win32/api/stringapiset/nf-stringapiset-widechartomultibyte) functions, controlled by feature `win32`. Since UTF-32 is not supported by these functions, [`widestring`](https://docs.rs/widestring) crate is used to convert UTF-32 to UTF-16 and vice versa.

You may also disable default features and enable `libiconv` to use the [libiconv] library instead.

### Linux

On Linux with glibc, the built-in [`iconv`](https://man7.org/linux/man-pages/man3/iconv.3.html) is used by default, controlled by feature `libc-iconv`. You may also disable default features and enable `libiconv` to use the [libiconv] library instead.

Other libcs may not have an `iconv` implementation that is compatible with glibc's (specifically the `//IGNORE` and `//TRANSLIT` extensions and proper BOM handling), hence `libc-iconv` feature does not apply to them. By default, `fallback-libiconv` feature applies and will link to the [libiconv] library. Make sure to have [libiconv] installed on user's system.

### macOS

Same as Linux with glibc.  You may also disable default features and enable `libiconv` to use the [libiconv] library instead.

### Web (WASM)

Uses [`TextDecoder`] and [`TextEncoder`] Web APIs. [`widestring`](https://docs.rs/widestring/latest/widestring/) crate is used to handle UTF-16 and UTF-32 related conversions.

<div class="warning">

As per [Encoding Standard], a standard-compliant browser supports only UTF-8 when using [`TextEncoder`], hence conversions **to** any encodings other than UTF-8/UTF-16/UTF-32 (including LE/BE variants) are not supported and will result in an `UnknownConversion` error.
Consider import a [polyfill](https://www.npmjs.com/package/text-encoding-polyfill) and enable `wasm-nonstandard-allow-legacy-encoding` feature if full encoding support is required, in which case 
most of the encodings will work. However, there is no guarantee as it is not a standard-compliant behavior.

Conversions **from** legacy encodings are not affected by this limitation. See [Encoding Standard] for more details.

</div>

[`TextDecoder`]: https://developer.mozilla.org/en-US/docs/Web/API/TextDecoder
[`TextEncoder`]: https://developer.mozilla.org/en-US/docs/Web/API/TextEncoder
[Encoding Standard]: https://encoding.spec.whatwg.org/#interface-textencoder
 
### Other

On other platforms, the [libiconv] library is used by default, controlled by feature `fallback-libiconv`.

## Feature flags

The following table summarizes the feature flags used to control the underlying implementation `iconv-native` uses on different platforms.

| Feature | Windows | GNU/Linux or GNU/Hurd, with glibc | macOS | Web (WASM) | Other |
|---------|---------|---------------------------------|-------|------------|-------|
| `win32` (default) | ✅       |                                   |       |            |       |
| `libc-iconv` (default) |         | ✅                             | ✅    |            |       |
| `web-encoding` (default) |         |                                   |       | ✅         |       |
| `libiconv` |  ✅       |   ✅                                |  ✅     |  ❓          | ✅    |
| `fallback-libiconv` (default) |  🉑       |   🉑                                |  🉑     |  ❓          |  🉑    |

- ✅: The corresponding implementation will take effect on the platform. For each platform, there can be **at most one** ✅ feature enabled.
- 🉑: The corresponding implementation will not take effect unless no ✅ feature is enabled on the platform.
- ❓: The corresponding implementation's applicability is not known on the platform.

The following optional feature flags can be used to control the behavior of certain implementations:

- `wasm-nonstandard-allow-legacy-encoding`: Enable this feature to allow legacy encodings other than UTF-8/UTF-16/UTF-32 (including LE/BE variants) on Web (WASM) platform. A polyfill is required for it to work.

[libiconv]: https://www.gnu.org/software/libiconv/
