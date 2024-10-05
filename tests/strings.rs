#![allow(dead_code)]

use cfg_if::cfg_if;

pub const TEST_GB18030: &[u8] = b"\xdc\xbd\xc4\xfe\xc4\xc8";
pub const TEST_UTF8: &[u8] = b"\xe8\x8a\x99\xe5\xae\x81\xe5\xa8\x9c";
pub const TEST_UTF8_BOM: &[u8] = b"\xef\xbb\xbf\xe8\x8a\x99\xe5\xae\x81\xe5\xa8\x9c";
pub const TEST_UTF16_LE: &[u8] = b"\x99\x82\x81\x5b\x1c\x5a";
pub const TEST_UTF16_LE_BOM: &[u8] = b"\xff\xfe\x99\x82\x81\x5b\x1c\x5a";
pub const TEST_UTF16_LE_BOM_2: &[u8] = b"\xff\xfe\xff\xfe\x99\x82\x81\x5b\x1c\x5a";
pub const TEST_UTF16_BE: &[u8] = b"\x82\x99\x5b\x81\x5a\x1c";
pub const TEST_UTF16_BE_BOM: &[u8] = b"\xfe\xff\x82\x99\x5b\x81\x5a\x1c";
pub const TEST_UTF16_BE_BOM_2: &[u8] = b"\xfe\xff\xfe\xff\x82\x99\x5b\x81\x5a\x1c";
pub const TEST_UTF32_LE: &[u8] = b"\x99\x82\0\0\x81\x5b\0\0\x1c\x5a\0\0";
pub const TEST_UTF32_LE_BOM: &[u8] = b"\xff\xfe\0\0\x99\x82\0\0\x81\x5b\0\0\x1c\x5a\0\0";
pub const TEST_UTF32_LE_BOM_2: &[u8] = b"\xff\xfe\xff\xfe\0\0\x99\x82\0\0\x81\x5b\0\0\x1c\x5a\0\0";
pub const TEST_UTF32_BE: &[u8] = b"\0\0\x82\x99\0\0\x5b\x81\0\0\x5a\x1c";
pub const TEST_UTF32_BE_BOM: &[u8] = b"\0\0\xfe\xff\0\0\x82\x99\0\0\x5b\x81\0\0\x5a\x1c";
pub const TEST_UTF32_BE_BOM_2: &[u8] =
    b"\0\0\xfe\xff\0\0\xfe\xff\0\0\x82\x99\0\0\x5b\x81\0\0\x5a\x1c";

// DE: default endianness
cfg_if! {
    if #[cfg(any(
        all(windows, feature = "win32"),
        all(target_arch = "wasm32", feature = "web-encoding"),
        all(
            feature = "libc-iconv",
            any(
                all(target_env = "gnu", target_os = "linux"),
                target_os = "hurd",
                target_vendor = "apple"
            )
        )
    ))] {
        pub const TEST_UTF16_DE: &[u8] = TEST_UTF16_LE;
        pub const TEST_UTF16_DE_BOM: &[u8] = TEST_UTF16_LE_BOM;
        pub const TEST_UTF16_DE_BOM_2: &[u8] = TEST_UTF16_LE_BOM_2;
        pub const TEST_UTF32_DE: &[u8] = TEST_UTF32_LE;
        pub const TEST_UTF32_DE_BOM: &[u8] = TEST_UTF32_LE_BOM;
        pub const TEST_UTF32_DE_BOM_2: &[u8] = TEST_UTF32_LE_BOM_2;
    } else {
        pub const TEST_UTF16_DE: &[u8] = TEST_UTF16_BE;
        pub const TEST_UTF16_DE_BOM: &[u8] = TEST_UTF16_BE_BOM;
        pub const TEST_UTF16_DE_BOM_2: &[u8] = TEST_UTF16_BE_BOM_2;
        pub const TEST_UTF32_DE: &[u8] = TEST_UTF32_BE;
        pub const TEST_UTF32_DE_BOM: &[u8] = TEST_UTF32_BE_BOM;
        pub const TEST_UTF32_DE_BOM_2: &[u8] = TEST_UTF32_BE_BOM_2;
    }
}
