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
