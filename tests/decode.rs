mod strings;

use iconv_native::*;
use strings::*;

#[test]
fn test_decode_lossy_success() {
    let testcases = [
        (TEST_UTF8, "utf-8"),
        (TEST_UTF16_LE, "utf-16"),
        (TEST_UTF16_BE_BOM, "utf-16"),
        (TEST_UTF16_LE, "utf-16le"),
        (TEST_UTF16_BE, "utf-16be"),
        (TEST_UTF16_LE_BOM, "utf-16le"),
        (TEST_UTF16_BE_BOM, "utf-16be"),
        (TEST_UTF32_LE, "utf-32"),
        (TEST_UTF32_BE_BOM, "utf-32"),
        (TEST_UTF32_LE, "utf-32le"),
        (TEST_UTF32_BE, "utf-32be"),
        (TEST_UTF32_LE_BOM, "utf-32le"),
        (TEST_UTF32_BE_BOM, "utf-32be"),
    ];
    let expected = "芙宁娜";
    for (idx, (input, encoding)) in testcases.into_iter().enumerate() {
        let result = decode_lossy(input, encoding).unwrap();
        assert_eq!(result, expected, "{idx}: {input:?} {encoding}");
    }
}

#[test]
fn test_decode_lossy_invalid_encoding() {
    let result = decode_lossy(TEST_GB18030, "invalid_encoding");
    assert_eq!(result, Err(ConvertLossyError::UnknownFromEncoding));
}
