Converts a byte sequence of `from_encoding` encoded text to `to_encoding`. Possibly includes invalid sequences or unrepresentable characters.

In case there are invalid sequences from `input` or characters that cannot be perfectly represented by `to_encoding`, the implementation will try to recover conversion at a best effort basis. The invalid sequences may result in replacement characters in the output or simply be ignored, depending on the underlying implementation.

# Byte order

For UTF-16 and UTF-32 without LE/BE, the byte order of `input` is determined by the BOM (Byte Order Mark) if it exists in the `input`. If the BOM is not present, a default byte order will be assumed which is implementation-defined.

For UTF-16 and UTF-32 without LE/BE, the output byte order is implementation-defined. 

# BOM handling

This function follows the same convention as [libiconv].

For input:

- If BOM is present and is used to determine the byte order for `UTF-16` or `UTF-32`, it will be removed.
- For `UTF-8`, `UTF-16LE`, `UTF-16BE`, `UTF-32LE` and `UTF-32BE`, the "BOM" is actually considered as `U+FEFF` (ZERO WIDTH NO-BREAK SPACE) in the text. It will be preserved.

For output:

- If `to_encoding` is `UTF-16` or `UTF-32`, a BOM will be added to the output.
- Otherwise, no BOM will be added. This includes `UTF-8`, `UTF-16LE`, `UTF-16BE`, `UTF-32LE` and `UTF-32BE`.

# Errors

- If `from_encoding`, `to_encoding` or the conversion pair from `from_encoding` to `to_encoding` is not supported by the underlying implementation, [`ConvertLossyError::UnknownConversion`] is returned.

# Examples

```rust
use iconv_native::{convert_lossy, ConvertLossyError};

let output_no_bom = convert_lossy(b"\xdc\xbd\xc4\xfe\xc4\xc8", "gb18030", "utf-16be");
let output_bom_added = convert_lossy(b"\xdc\xbd\xc4\xfe\xc4\xc8", "gb18030", "utf-16");
let output_bom_removed = convert_lossy(
    b"\xff\xfe\x66\x8e\x66\x8e\xb8\x70\x39\x5f",
    "utf-16",
    "utf-32le",
);
let output_bom_inout = convert_lossy(
    b"\xfe\xff\x8e\x66\x8e\x66\x70\xb8\x5f\x39",
    "utf-16",
    "utf-32",
);
let output_not_a_bom = convert_lossy(
    b"\xef\xbb\xbf\xe8\x8a\x99\xe5\xae\x81\xe5\xa8\x9c",
    "utf-8",
    "utf-32be",
);
let output_invalid_encoding = convert_lossy(b"\x11\x45", "invalid-encoding", "utf-8");
let output_invalid_input = convert_lossy(b"\xff 141919", "utf-8", "gb18030");

const EXPECTED_NO_BOM: &[u8] = b"\x82\x99\x5b\x81\x5a\x1c";
const EXPECTED_BOM_ADDED_BE: &[u8] = b"\xfe\xff\x82\x99\x5b\x81\x5a\x1c";
const EXPECTED_BOM_ADDED_LE: &[u8] = b"\xff\xfe\x99\x82\x81\x5b\x1c\x5a";
const EXPECTED_BOM_REMOVED: &[u8] = b"\x66\x8e\0\0\x66\x8e\0\0\xb8\x70\0\0\x39\x5f\0\0";
const EXPECTED_BOM_INOUT_BE: &[u8] =
    b"\xff\xfe\0\0\x66\x8e\0\0\x66\x8e\0\0\xb8\x70\0\0\x39\x5f\0\0";
const EXPECTED_BOM_INOUT_LE: &[u8] =
    b"\0\0\xfe\xff\0\0\x8e\x66\0\0\x8e\x66\0\0\x70\xb8\0\0\x5f\x39";
const EXPECTED_NOT_A_BOM: &[u8] = b"\0\0\xfe\xff\0\0\x82\x99\0\0\x5b\x81\0\0\x5a\x1c";

assert_eq!(output_no_bom?, EXPECTED_NO_BOM, "no bom");
assert!(
    matches!(
        &*output_bom_added?,
        EXPECTED_BOM_ADDED_BE | EXPECTED_BOM_ADDED_LE
    ),
    "bom added"
);
assert_eq!(output_bom_removed?, EXPECTED_BOM_REMOVED, "bom removed");
assert!(
    matches!(
        &*output_bom_inout?,
        EXPECTED_BOM_INOUT_BE | EXPECTED_BOM_INOUT_LE
    ),
    "bom in-out"
);
assert_eq!(output_not_a_bom?, EXPECTED_NOT_A_BOM, "not a bom");
assert_eq!(
    output_invalid_encoding.unwrap_err(),
    ConvertLossyError::UnknownConversion
);
assert!(output_invalid_input?.iter().rev().take(6).eq(b"919141"));
# Ok::<(), iconv_native::ConvertLossyError>(())
```

[libiconv]: https://www.gnu.org/software/libiconv/
