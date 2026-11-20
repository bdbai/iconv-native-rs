#[macro_use]
mod harness;
mod strings;

use iconv_native::*;
use strings::*;

with_harness! {
    fn test_decode_success() {
        let testcases = [
            (TEST_UTF8, "utf-8"),
            (TEST_UTF8_BOM, "utf-8"),
            (TEST_UTF16_DE, "utf-16"),
            (TEST_UTF16_BE_BOM, "utf-16"),
            (TEST_UTF16_LE, "utf-16le"),
            (TEST_UTF16_BE, "utf-16be"),
            (TEST_UTF16_LE_BOM, "utf-16le"),
            (TEST_UTF16_BE_BOM, "utf-16be"),
            (TEST_UTF32_DE, "utf-32"),
            (TEST_UTF32_BE_BOM, "utf-32"),
            (TEST_UTF32_LE, "utf-32le"),
            (TEST_UTF32_BE, "utf-32be"),
            (TEST_UTF32_LE_BOM, "utf-32le"),
            (TEST_UTF32_BE_BOM, "utf-32be"),
        ];
        let expected = "芙宁娜";
        for (idx, (input, encoding)) in testcases.into_iter().enumerate() {
            let result = decode(input, encoding).unwrap();
            assert_eq!(result, expected, "{idx}: {input:?} {encoding}");

            let result = decode_lossy(input, encoding).unwrap();
            assert_eq!(result, expected, "{idx}_lossy: {input:?} {encoding}");
        }
    }

    fn test_decode_empty_input() {
        let result = decode("", "utf-16le");
        let result_lossy = decode_lossy("", "utf-16le");
        assert_eq!(result, Ok("".into()), "utf-16le");
        assert_eq!(result_lossy, Ok("".into()), "utf-16le lossy");

        let result = decode("", "gb18030");
        let result_lossy = decode_lossy("", "gb18030");
        assert_eq!(result, Ok("".into()), "gb18030");
        assert_eq!(result_lossy, Ok("".into()), "gb18030 lossy");
    }

    fn test_decode_bom_resume() {
        // Long ambiguous UTF-16 input with BOM; forces E2BIG resume with the
        // small initial buffer. BOM is handled upfront so the tail decodes
        // with the right byte order.
        let mut input = b"\xff\xfe".to_vec();
        for _ in 0..64 {
            input.extend_from_slice(b"\x39\x5f");
        }
        let expected = "\u{5f39}".repeat(64);
        let result = decode(&input, "utf-16").unwrap();
        assert_eq!(result, expected);
        let result_lossy = decode_lossy(&input, "utf-16").unwrap();
        assert_eq!(result_lossy, expected, "lossy");
    }

    // See test_convert_bom_unicode_alias: iconv backends only.
    #[cfg(not(all(windows, feature = "win32")))]
    #[cfg(not(all(target_arch = "wasm32", feature = "web-encoding")))]
    fn test_decode_bom_unicode_alias() {
        let mut input = b"\xff\xfe".to_vec();
        for _ in 0..16 {
            input.extend_from_slice(b"\x39\x5f");
        }
        let expected = "\u{5f39}".repeat(16);
        let result = decode(&input, "unicode").unwrap();
        assert_eq!(result, expected);
        let result_lossy = decode_lossy(&input, "unicode").unwrap();
        assert_eq!(result_lossy, expected, "lossy");
    }

    fn test_decode_invalid_input() {
        let result = decode(TEST_GB18030, "utf-8");
        assert_eq!(result, Err(ConvertError::InvalidInput));
    }

    fn test_decode_invalid_encoding() {
        let result = decode(TEST_GB18030, "invalid_encoding");
        assert_eq!(result, Err(ConvertError::UnknownConversion));
        let result = decode_lossy(TEST_GB18030, "invalid_encoding");
        assert_eq!(result, Err(ConvertLossyError::UnknownConversion));
    }
}
