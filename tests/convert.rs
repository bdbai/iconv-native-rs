mod strings;
#[macro_use]
mod harness;

use iconv_native::*;
use strings::*;

// TextEncoder does not support legacy encodings
#[test]
fn test_convert_lossy_success() {
    let input = TEST_GB18030;
    let result = convert_lossy(input, "gb18030", "utf-8").unwrap();
    let result_rev = convert_lossy(&result, "utf-8", "gb18030").unwrap();
    assert_eq!(result, TEST_UTF8);
    assert_eq!(result_rev, input);
}

with_harness! {
    fn test_convert_lossy_roundtrip() {
        let testcases = [
            (TEST_UTF8, &TEST_UTF16_LE[..], "utf-8", "utf-16le"),
            (TEST_UTF8_BOM, TEST_UTF16_LE_BOM, "utf-8", "utf-16le"),
            (TEST_UTF8, TEST_UTF16_BE, "utf-8", "utf-16be"),
            (TEST_UTF8_BOM, TEST_UTF16_BE_BOM, "utf-8", "utf-16be"),
            (TEST_UTF8, TEST_UTF32_LE, "utf-8", "utf-32le"),
            (TEST_UTF8_BOM, TEST_UTF32_LE_BOM, "utf-8", "utf-32le"),
            (TEST_UTF8, TEST_UTF32_BE, "utf-8", "utf-32be"),
            (TEST_UTF8_BOM, TEST_UTF32_BE_BOM, "utf-8", "utf-32be"),
            (TEST_UTF8_BOM, TEST_UTF16_LE_BOM, "utf-8", "utf-16le"),
            (TEST_UTF16_DE_BOM, TEST_UTF32_DE_BOM, "utf-16", "utf-32"),
        ];
        for (idx, (input, expected, from_encoding, to_encoding)) in testcases.into_iter().enumerate() {
            let result = convert_lossy(input, from_encoding, to_encoding).unwrap();
            let result_rev = convert_lossy(&result, to_encoding, from_encoding).unwrap();
            assert_eq!(
                result, expected,
                "{idx}: {input:?} {from_encoding} => {expected:?} {to_encoding}"
            );
            assert_eq!(
                result_rev, input,
                "{idx}: {input:?} {from_encoding} <= {expected:?} {to_encoding}"
            );
        }
    }

    fn test_convert_lossy_bom() {
        let testcases_be_16 = [
            (TEST_UTF8_BOM, &TEST_UTF16_DE_BOM_2[..], "utf-8", "utf-16"),
            (TEST_UTF8, TEST_UTF16_DE_BOM, "utf-8", "utf-16"),
            (TEST_UTF16_DE_BOM, TEST_UTF8, "utf-16", "utf-8"),
            (TEST_UTF16_DE, TEST_UTF8, "utf-16", "utf-8"),
            (TEST_UTF16_DE, TEST_UTF32_DE_BOM, "utf-16", "utf-32"),
        ];
        let testcases_be_32 = [
            (TEST_UTF8_BOM, &TEST_UTF32_DE_BOM_2[..], "utf-8", "utf-32"),
            (TEST_UTF8, TEST_UTF32_DE_BOM, "utf-8", "utf-32"),
            (TEST_UTF32_DE_BOM, TEST_UTF8, "utf-32", "utf-8"),
            (TEST_UTF32_DE, TEST_UTF8, "utf-32", "utf-8"),
            (TEST_UTF32_DE, TEST_UTF16_DE_BOM, "utf-32", "utf-16"),
        ];
        let testcases = [
            ("be_16", &testcases_be_16[..]),
            ("be_32", &testcases_be_32[..]),
        ];
        for (casename, testcases) in testcases {
            for (idx, (input, expected, from_encoding, to_encoding)) in
                testcases.into_iter().cloned().enumerate()
            {
                let result = convert_lossy(input, from_encoding, to_encoding).unwrap();
                assert_eq!(
                    result, expected,
                    "{casename}_{idx}: {input:?} {from_encoding} => {expected:?} {to_encoding}"
                );
            }
        }
    }

    fn test_convert_lossy_same_encoding() {
        let input = TEST_UTF8;
        let result = convert_lossy(input, "utf-8", "utf-8").unwrap();
        assert_eq!(result, input);
    }

    fn test_convert_lossy_invalid_from_encoding() {
        let result = convert_lossy(TEST_GB18030, "invalid_encoding", "utf-8");
        assert_eq!(result, Err(ConvertLossyError::UnknownConversion));
    }

    fn test_convert_lossy_invalid_to_encoding() {
        let result = convert_lossy(TEST_GB18030, "gb18030", "invalid_encoding");
        assert_eq!(result, Err(ConvertLossyError::UnknownConversion));
    }

    fn test_convert_lossy_invalid_input() {
        let result = convert_lossy(b"b\xffaa", "utf-8", "utf-16").unwrap();
        assert!(result.contains(&b'a'));
    }
}
