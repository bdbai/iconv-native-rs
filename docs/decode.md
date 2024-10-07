Converts text represented by a slice of bytes of a specified encoding to a [`String`].

# Byte order

For UTF-16 and UTF-32 without LE/BE, the byte order is determined by the BOM (Byte Order Mark) if it exists in the `input`. If the BOM is not present, a default byte order will be assumed which is implementation-defined.

# BOM handling

This function will remove the UTF-8 BOM (`b"\xef\xbb\xbf"`) from `output` unconditionally if it exists.

# Errors

- If `encoding` or the conversion pair from `encoding` to UTF-8 is not supported by the underlying implementation, [`ConvertError::UnknownConversion`] is returned.
- If `input` contains invalid sequences for `encoding`, [`ConvertError::InvalidInput`] is returned. If you do not care about them, use [`decode_lossy`](fn.decode_lossy.html) instead. Note that some implementations may not report errors in this case and will return the result with replacement characters.

# Examples

```rust
use iconv_native::{decode, ConvertError};

let output = decode(b"\xdc\xbd\xc4\xfe\xc4\xc8", "gb18030");
let output_bom = decode(b"\xff\xfe\x66\x8e\x66\x8e\xb8\x70\x39\x5f", "utf-16");
let output_invalid_encoding = decode(b"\x11\x45", "invalid-encoding");
let output_invalid_input = decode(b"\xff 141919", "utf-8");

assert_eq!(output?, "芙宁娜");
assert_eq!(output_bom?, "蹦蹦炸弹");
assert_eq!(
    output_invalid_encoding.unwrap_err(),
    ConvertError::UnknownConversion
);
assert_eq!(
    output_invalid_input.unwrap_err(),
    ConvertError::InvalidInput
);
# Ok::<(), iconv_native::ConvertError>(())
```
