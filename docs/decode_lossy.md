Converts text represented by a slice of bytes of a specified encoding to a [`String`]. Possibly includes invalid sequences.

In case there are invalid sequences from `input`, the implementation will try to recover conversion at a best effort basis. The invalid sequences may result in replacement characters in the output or simply be ignored, depending on the underlying implementation.

# Byte order

For UTF-16 and UTF-32 without LE/BE, the byte order is determined by the BOM (Byte Order Mark) if it exists in the `input`. If the BOM is not present, a default byte order will be assumed which is implementation-defined.

# BOM handling

This function will remove the UTF-8 BOM (`b"\xef\xbb\xbf"`) from `output` unconditionally if it exists.

# Errors

- If `encoding` or the conversion pair from `encoding` to UTF-8 is not supported by the underlying implementation, [`ConvertError::UnknownConversion`] is returned.

# Examples

```rust
use iconv_native::{decode_lossy, ConvertLossyError};

let output = decode_lossy(b"\xdc\xbd\xc4\xfe\xc4\xc8", "gb18030");
let output_bom = decode_lossy(b"\xff\xfe\x66\x8e\x66\x8e\xb8\x70\x39\x5f", "utf-16");
let output_invalid_encoding = decode_lossy(b"\x11\x45", "invalid-encoding");
let output_invalid_input = decode_lossy(b"\xff 141919", "utf-8");

assert_eq!(output?, "芙宁娜");
assert_eq!(output_bom?, "蹦蹦炸弹");
assert_eq!(
    output_invalid_encoding.unwrap_err(),
    ConvertLossyError::UnknownConversion
);
assert!(output_invalid_input?.contains("141919"));
# Ok::<(), iconv_native::ConvertLossyError>(())
```
