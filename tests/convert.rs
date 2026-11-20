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

    fn test_convert_bom_resume() {
        // Regression test: the output (256 bytes) is much larger than the
        // initial output buffer (input.len() + 4), forcing the conversion to
        // resume after E2BIG. The BOM is stripped upfront and the source is
        // rewritten to an explicit LE/BE encoding, so the byte order stays
        // correct across resumed calls on all platforms.
        let mut input = b"\xff\xfe".to_vec();
        let mut expected = Vec::new();
        for _ in 0..64 {
            input.extend_from_slice(b"\x39\x5f");
            expected.extend_from_slice(b"\x39\x5f\0\0");
        }
        let result = convert(&input, "utf-16", "utf-32le").unwrap();
        assert_eq!(result, expected);
        let result_lossy = convert_lossy(&input, "utf-16", "utf-32le").unwrap();
        assert_eq!(result_lossy, expected, "lossy");
    }

    // The `unicode` alias maps to ambiguous UTF-16 and needs the same BOM
    // handling. Only exercised on iconv backends; win32/wasm decode it
    // through a different path.
    #[cfg(not(all(windows, feature = "win32")))]
    #[cfg(not(all(target_arch = "wasm32", feature = "web-encoding")))]
    fn test_convert_bom_unicode_alias() {
        let mut input_le = b"\xff\xfe".to_vec();
        let mut input_be = b"\xfe\xff".to_vec();
        let mut expected = Vec::new();
        for _ in 0..16 {
            input_le.extend_from_slice(b"\x39\x5f");
            input_be.extend_from_slice(b"\x5f\x39");
            expected.extend_from_slice(b"\x39\x5f\0\0");
        }
        for from in ["unicode", "UNICODE"] {
            let result = convert(&input_le, from, "utf-32le").unwrap();
            assert_eq!(result, expected, "le {from}");
            let result_lossy = convert_lossy(&input_le, from, "utf-32le").unwrap();
            assert_eq!(result_lossy, expected, "le lossy {from}");
            let result = convert(&input_be, from, "utf-32le").unwrap();
            assert_eq!(result, expected, "be {from}");
            let result_lossy = convert_lossy(&input_be, from, "utf-32le").unwrap();
            assert_eq!(result_lossy, expected, "be lossy {from}");
        }
    }

    // UTF-32 with BOM over a long input exercises the same upfront BOM
    // handling for the 4-byte BOM variants.
    #[cfg(not(all(windows, feature = "win32")))]
    #[cfg(not(all(target_arch = "wasm32", feature = "web-encoding")))]
    fn test_convert_bom_utf32_long() {
        let mut input_le = b"\xff\xfe\0\0".to_vec();
        let mut input_be = b"\0\0\xfe\xff".to_vec();
        let mut expected_utf8 = Vec::new();
        let mut expected_utf16le = Vec::new();
        for _ in 0..64 {
            input_le.extend_from_slice(b"\x39\x5f\0\0");
            input_be.extend_from_slice(b"\0\0\x5f\x39");
            expected_utf8.extend_from_slice(b"\xe5\xbc\xb9");
            expected_utf16le.extend_from_slice(b"\x39\x5f");
        }
        for (input, name) in [(&input_le[..], "le"), (&input_be[..], "be")] {
            let result = convert(input, "utf-32", "utf-8").unwrap();
            assert_eq!(result, expected_utf8, "{name}");
            let result_lossy = convert_lossy(input, "utf-32", "utf-8").unwrap();
            assert_eq!(result_lossy, expected_utf8, "{name} lossy");
            let result = convert(input, "utf-32", "utf-16le").unwrap();
            assert_eq!(result, expected_utf16le, "{name} utf16");
        }
    }

    // UTF-8-MAC decomposes Hangul syllables, so the output can exceed
    // `input.len() * 4 + 32` and still hit E2BIG. The upfront BOM handling
    // keeps the byte order correct. Apple system iconv only.
    #[cfg(all(target_vendor = "apple", feature = "libc-iconv"))]
    fn test_convert_utf16_to_utf8mac_expansion() {
        let mut input = b"\xff\xfe".to_vec();
        for _ in 0..41 {
            input.extend_from_slice(b"\x01\xac");
        }
        let mut expected = Vec::new();
        for _ in 0..41 {
            expected.extend_from_slice(b"\xe1\x84\x80\xe1\x85\xa1\xe1\x86\xa8");
        }
        match convert(&input, "utf-16", "utf-8-mac") {
            Ok(result) => assert_eq!(result, expected),
            Err(ConvertError::UnknownConversion) => return,
            Err(e) => panic!("unexpected error: {e:?}"),
        }
        match convert_lossy(&input, "utf-16", "utf-8-mac") {
            Ok(result) => assert_eq!(result, expected, "lossy"),
            Err(ConvertLossyError::UnknownConversion) => {}
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
