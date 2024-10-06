use core::str::FromStr;

use cfg_if::cfg_if;

cfg_if! {
    if #[cfg(any(
        all(windows, feature = "win32"),
        all(target_arch = "wasm32", feature = "web-encoding"),
    ))] {
        mod decode_lossy;
        pub(crate) use decode_lossy::decode_utf_lossy;
        mod decode;
        pub(crate) use decode::decode_utf;
    }
}

use crate::bom::{ByteOrderMark, ByteOrderMarkExt};
use crate::encoding::trim_encoding_prefix;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum UtfType {
    Utf8,
    Utf16,
    Utf32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct UtfEncoding {
    r#type: UtfType,
    byte_order: ByteOrderMark,
}

impl UtfEncoding {
    fn parse(s: &str) -> Option<Self> {
        let encoding = trim_encoding_prefix(s, "utf")?;
        let (r#type, byte_order) = if encoding.starts_with("8") {
            (UtfType::Utf8, trim_encoding_prefix(encoding, "8"))
        } else if encoding.starts_with("16") {
            (UtfType::Utf16, trim_encoding_prefix(encoding, "16"))
        } else if encoding.starts_with("32") {
            (UtfType::Utf32, trim_encoding_prefix(encoding, "32"))
        } else {
            return None;
        };
        let byte_order = match byte_order {
            Some("le" | "LE") => ByteOrderMark::Le,
            Some("be" | "BE") => ByteOrderMark::Be,
            Some("") => ByteOrderMark::NotPresent,
            _ => return None,
        };
        Some(Self { r#type, byte_order })
    }

    pub(crate) fn is_ambiguous(&self) -> bool {
        !self.is_utf8() && self.byte_order == ByteOrderMark::NotPresent
    }

    #[allow(dead_code)]
    pub(crate) fn r#type(&self) -> UtfType {
        self.r#type
    }

    #[allow(dead_code)]
    pub(crate) fn byte_order(&self) -> ByteOrderMark {
        self.byte_order
    }

    #[allow(dead_code)]
    pub(crate) fn is_utf8(&self) -> bool {
        self.r#type == UtfType::Utf8
    }

    #[allow(dead_code)]
    pub(crate) fn is_utf16(&self) -> bool {
        self.r#type == UtfType::Utf16
    }

    #[allow(dead_code)]
    pub(crate) fn is_utf32(&self) -> bool {
        self.r#type == UtfType::Utf32
    }

    pub(crate) fn strip_bom(&self, input: &mut &[u8]) -> bool {
        match (self.r#type, self.byte_order) {
            (UtfType::Utf8, _) if input.get_utf8_bom().is_present() => {
                *input = &input[3..];
            }
            (UtfType::Utf16, byte_order)
                if byte_order.is_present() && input.get_utf16_bom() == byte_order =>
            {
                *input = &input[2..];
            }
            (UtfType::Utf32, byte_order)
                if byte_order.is_present() && input.get_utf32_bom() == byte_order =>
            {
                *input = &input[4..];
            }
            _ => return false,
        };
        true
    }

    #[allow(dead_code)]
    pub(crate) fn consume_input_bom(&self, input: &mut &[u8]) -> ByteOrderMark {
        if !self.is_ambiguous() {
            return self.byte_order;
        }
        let (input_bom, bom_len) = match self.r#type {
            UtfType::Utf16 => {
                let bom = input.get_utf16_bom();
                (bom, 2)
            }
            UtfType::Utf32 => {
                let bom = input.get_utf32_bom();
                (bom, 4)
            }
            // Consider UTF-8 "BOM" as a part of the input. Do not consume it.
            _ => return ByteOrderMark::NotPresent,
        };
        if input_bom.is_present() {
            *input = &input[bom_len..];
            input_bom
        } else {
            self.byte_order
        }
    }
}

impl FromStr for UtfEncoding {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::parse(s).ok_or(())
    }
}

#[cfg(test)]
mod tests {
    use alloc::string::ToString;

    use super::*;

    #[test]
    fn test_utf_encoding_parse() {
        let testcases = [
            ("utf-8", UtfType::Utf8, ByteOrderMark::NotPresent),
            ("utf-16", UtfType::Utf16, ByteOrderMark::NotPresent),
            ("utf-32", UtfType::Utf32, ByteOrderMark::NotPresent),
            ("utf-16le", UtfType::Utf16, ByteOrderMark::Le),
            ("utf-16be", UtfType::Utf16, ByteOrderMark::Be),
            ("utf-32le", UtfType::Utf32, ByteOrderMark::Le),
            ("utf-32be", UtfType::Utf32, ByteOrderMark::Be),
        ];
        for (input, r#type, byte_order) in testcases.iter() {
            let encoding = UtfEncoding::parse(input).expect(&input);
            assert_eq!(encoding.r#type, *r#type, "{input}");
            assert_eq!(encoding.byte_order, *byte_order, "{input}");
        }
    }

    #[test]
    fn test_utf_encoding_parse_invalid() {
        let testcases = ["utf", "utf-9", "utf-16lebe", "wtf-32"];
        for input in testcases.iter() {
            let encoding = UtfEncoding::parse(input);
            assert!(encoding.is_none(), "{input}");
        }
    }

    #[test]
    fn test_utf_encoding_is_ambiguous() {
        let testcases = [
            ("utf-8", false),
            ("utf-16", true),
            ("utf-32", true),
            ("utf-16le", false),
            ("utf-16be", false),
            ("utf-32le", false),
            ("utf-32be", false),
        ];
        for (input, expected) in testcases.iter() {
            let encoding = UtfEncoding::parse(input).expect(&input);
            assert_eq!(encoding.is_ambiguous(), *expected, "{input}");
        }
    }

    #[test]
    fn test_utf_encoding_consume_input_bom() {
        let testcases = [
            (
                "utf-8",
                &b"hello"[..],
                ByteOrderMark::NotPresent,
                &b"hello"[..],
            ),
            ("utf-16", b"hello", ByteOrderMark::NotPresent, b"hello"),
            ("utf-16", b"\xff\xfehello", ByteOrderMark::Le, b"hello"),
            ("utf-16", b"\xfe\xffhello", ByteOrderMark::Be, b"hello"),
            ("utf-16be", b"hello", ByteOrderMark::Be, b"hello"),
            (
                "utf-16be",
                b"\xff\xfehello",
                ByteOrderMark::Be,
                b"\xff\xfehello",
            ),
            (
                "utf-16be",
                b"\xfe\xffhello",
                ByteOrderMark::Be,
                b"\xfe\xffhello",
            ),
            ("utf-16le", b"hello", ByteOrderMark::Le, b"hello"),
            (
                "utf-16le",
                b"\xff\xfehello",
                ByteOrderMark::Le,
                b"\xff\xfehello",
            ),
            (
                "utf-16le",
                b"\xfe\xffhello",
                ByteOrderMark::Le,
                b"\xfe\xffhello",
            ),
            ("utf-32", b"hello", ByteOrderMark::NotPresent, b"hello"),
            (
                "utf-32",
                b"\xff\xfe\x00\x00hello",
                ByteOrderMark::Le,
                b"hello",
            ),
            (
                "utf-32",
                b"\x00\x00\xfe\xffhello",
                ByteOrderMark::Be,
                b"hello",
            ),
            ("utf-32be", b"hello", ByteOrderMark::Be, b"hello"),
            (
                "utf-32be",
                b"\xff\xfe\x00\x00hello",
                ByteOrderMark::Be,
                b"\xff\xfe\x00\x00hello",
            ),
            (
                "utf-32be",
                b"\x00\x00\xfe\xffhello",
                ByteOrderMark::Be,
                b"\x00\x00\xfe\xffhello",
            ),
            ("utf-32le", b"hello", ByteOrderMark::Le, b"hello"),
            (
                "utf-32le",
                b"\xff\xfe\x00\x00hello",
                ByteOrderMark::Le,
                b"\xff\xfe\x00\x00hello",
            ),
            (
                "utf-32le",
                b"\x00\x00\xfe\xffhello",
                ByteOrderMark::Le,
                b"\x00\x00\xfe\xffhello",
            ),
        ];
        for (idx, (encoding, input, expected_bom, expected_input)) in
            testcases.into_iter().enumerate()
        {
            let idx = idx.to_string();
            let mut input = input;
            let encoding = UtfEncoding::parse(encoding).expect(&idx);
            let bom = encoding.consume_input_bom(&mut input);
            assert_eq!(bom, expected_bom, "{idx}");
            assert_eq!(input, expected_input, "{idx}");
        }
    }
}
