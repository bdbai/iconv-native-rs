mod strings;
#[macro_use]
mod harness;

use iconv_native::*;
use strings::*;

// TextEncoder does not support legacy encodings
#[test]
fn test_convert_success() {
    let input = TEST_GB18030;

    let result = convert(input, "gb18030", "utf-8").unwrap();
    let result_rev = convert(&result, "utf-8", "gb18030").unwrap();
    assert_eq!(result, TEST_UTF8);
    assert_eq!(result_rev, input, "rev");

    let result = convert_lossy(input, "gb18030", "utf-8").unwrap();
    let result_rev = convert_lossy(&result, "utf-8", "gb18030").unwrap();
    assert_eq!(result, TEST_UTF8, "lossy");
    assert_eq!(result_rev, input, "lossy rev");
}

with_harness! {
    fn test_convert_roundtrip() {
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
        for (idx, (input, expected, from_encoding, to_encoding)) in
            testcases.into_iter().enumerate()
        {
            let result = convert(input, from_encoding, to_encoding).unwrap();
            let result_rev = convert(&result, to_encoding, from_encoding).unwrap();
            assert_eq!(
                result, expected,
                "{idx}: {input:?} {from_encoding} => {expected:?} {to_encoding}"
            );
            assert_eq!(
                result_rev, input,
                "{idx}: {input:?} {from_encoding} <= {expected:?} {to_encoding}"
            );

            let result = convert_lossy(input, from_encoding, to_encoding).unwrap();
            let result_rev = convert_lossy(&result, to_encoding, from_encoding).unwrap();
            assert_eq!(
                result, expected,
                "{idx}_lossy: {input:?} {from_encoding} => {expected:?} {to_encoding}"
            );
            assert_eq!(
                result_rev, input,
                "{idx}_lossy: {input:?} {from_encoding} <= {expected:?} {to_encoding}"
            );
        }
    }

    fn test_convert_bom() {
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
                let result = convert(input, from_encoding, to_encoding).unwrap();
                assert_eq!(
                    result, expected,
                    "{casename}_{idx}: {input:?} {from_encoding} => {expected:?} {to_encoding}"
                );

                let result = convert_lossy(input, from_encoding, to_encoding).unwrap();
                assert_eq!(
                    result, expected,
                    "{casename}_{idx}_lossy: {input:?} {from_encoding} => {expected:?} {to_encoding}"
                );
            }
        }
    }

    fn test_convert_same_encoding() {
        let input = TEST_UTF8;

        let result = convert(input, "utf-8", "utf-8").unwrap();
        assert_eq!(result, input);

        let result = convert_lossy(input, "utf-8", "utf-8").unwrap();
        assert_eq!(result, input, "lossy");
    }

    fn test_convert_empty_input() {
        let result = convert("", "utf-8", "utf-16le");
        let result_lossy = convert_lossy("", "utf-8", "utf-16le");
        assert_eq!(result, Ok(vec![]), "utf-8");
        assert_eq!(result_lossy, Ok(vec![]), "utf-8 lossy");

        let result = convert("", "gb18030", "utf-8");
        let result_lossy = convert_lossy("", "gb18030", "utf-8");
        assert_eq!(result, Ok(vec![]), "gb18030");
        assert_eq!(result_lossy, Ok(vec![]), "gb18030 lossy");
    }

    fn test_convert_invalid_from_encoding() {
        let result = convert(TEST_GB18030, "invalid_encoding", "utf-8");
        assert_eq!(result, Err(ConvertError::UnknownConversion));

        let result = convert_lossy(TEST_GB18030, "invalid_encoding", "utf-8");
        assert_eq!(result, Err(ConvertLossyError::UnknownConversion), "lossy");
    }

    fn test_convert_invalid_to_encoding() {
        let result = convert(TEST_GB18030, "gb18030", "invalid_encoding");
        assert_eq!(result, Err(ConvertError::UnknownConversion));

        let result = convert_lossy(TEST_GB18030, "gb18030", "invalid_encoding");
        assert_eq!(result, Err(ConvertLossyError::UnknownConversion));
    }

    fn test_convert_invalid_input() {
        let testcases = [
            (&b"b\xffaa"[..], "utf-8", "utf-16", &b"a"[..]),
            (&TEST_UTF16_DE[..3], "utf-16", "utf-8", b"\xe8\x8a\x99"),
            (b"\0\xd8\x99\x82", "utf-16le", "utf-32", b"\x99\x82"),
            (b"\xff\xdc\xbd", "gb18030", "utf-8", b"\xe8\x8a\x99"),
        ];
        for (idx, (input, from_encoding, to_encoding, expected_lossy_bytes)) in
            testcases.into_iter().enumerate()
        {
            let idx = idx.to_string();
            let result = convert(input, from_encoding, to_encoding);
            let result_lossy =
                convert_lossy(input, from_encoding, to_encoding).expect(&(idx.clone() + " lossy"));
            assert_eq!(
                result.expect_err(&(idx.clone() + " invalid input")),
                ConvertError::InvalidInput,
                "{idx}"
            );
            for expected_lossy_char in expected_lossy_bytes {
                assert!(result_lossy.contains(expected_lossy_char), "{idx} lossy");
            }
        }
    }

    // `WideCharToMultiByte` supports a `WC_ERR_INVALID_CHARS` flag, but it only works for UTF-8 and
    // GB18030. Really don't know how to let it fail.
    #[cfg(not(all(windows, feature = "win32")))]
    // A standard TextEncoder does not support legacy encodings.
    #[cfg(not(all(target_arch = "wasm32", feature = "web-encoding")))]
    fn test_convert_out_of_range() {
        let result = convert("🤣b", "utf-8", "iso-8859-1");
        let result_lossy = convert_lossy("🤣b", "utf-8", "iso-8859-1").unwrap();
        assert_eq!(result.unwrap_err(), ConvertError::InvalidInput);
        assert!(result_lossy.contains(&b'b'), "lossy");
    }
}
