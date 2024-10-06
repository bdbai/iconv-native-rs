use core::ops::RangeInclusive;

use crate::encoding::{match_encoding_parts, match_encoding_parts_exact, trim_encoding_prefix};

pub(super) const CODEPAGE_UTF8: u32 = 65001;
pub(super) const CODEPAGE_UTF16: u32 = 1200;
pub(super) const CODEPAGE_UTF16BE: u32 = 1201;
pub(super) const CODEPAGE_UTF32: u32 = 12000;
pub(super) const CODEPAGE_UTF32BE: u32 = 12001;

pub(super) fn encoding_to_codepage(encoding: &str) -> Option<u32> {
    if encoding.find(|c: char| !c.is_ascii()).is_some() {
        return None;
    }

    if match_encoding_parts_exact(encoding, &["cp", "1025"]) {
        return Some(21025);
    }
    if encoding.eq_ignore_ascii_case("csISO2022JP") {
        return Some(50221);
    }

    if let Some(encoding) = ["asmo", "dos", "windows", "cp"]
        .into_iter()
        .find_map(|prefix| trim_encoding_prefix(encoding, prefix))
        .or_else(|| match_encoding_parts(encoding, &["x", "cp"]))
    {
        return encoding.parse().ok();
    }

    if let Some(encoding) = trim_encoding_prefix(encoding, "euc") {
        match encoding {
            _ if encoding.eq_ignore_ascii_case("cn") => return Some(51936),
            "JP" => return Some(20932),
            _ if encoding.eq_ignore_ascii_case("jp") => return Some(51932),
            _ if encoding.eq_ignore_ascii_case("kr") => return Some(51949),
            _ => return None,
        }
    }
    if let Some(encoding) = trim_encoding_prefix(encoding, "utf") {
        match encoding {
            "7" => return Some(65000),
            "8" => return Some(CODEPAGE_UTF8),
            _ => {}
        }
        let digits = encoding.get(..2)?;
        let byte_order =
            trim_encoding_prefix(encoding, digits).expect("utf encoding without digits");
        match (digits, byte_order) {
            ("16", "le" | "LE" | "") => return Some(CODEPAGE_UTF16),
            ("32", "le" | "LE" | "") => return Some(CODEPAGE_UTF32),
            ("16", "be" | "BE") => return Some(CODEPAGE_UTF16BE),
            ("32", "be" | "BE") => return Some(CODEPAGE_UTF32BE),
            _ => return None,
        }
    }
    if let Some(encoding) = trim_encoding_prefix(encoding, "gb") {
        match encoding {
            "k" | "K" | "2312" => return Some(936),
            "18030" => return Some(54936),
            _ => return None,
        }
    }
    if let Some(encoding) = trim_encoding_prefix(encoding, "ibm") {
        if encoding.eq_ignore_ascii_case("thai") {
            return Some(20838);
        }
        let digits: u32 = encoding.parse().ok()?;
        return Some(if let 924 | 273..=424 | 871..=905 = digits {
            digits + 20000
        } else {
            digits
        });
    }
    if let Some(encoding) = trim_encoding_prefix(encoding, "iso") {
        if let Some(encoding) = trim_encoding_prefix(encoding, "2022") {
            match encoding {
                "jp" => return Some(50220),
                "kr" => return Some(50225),
                _ => return None,
            }
        }
        if let Some(encoding) = trim_encoding_prefix(encoding, "8859") {
            match encoding {
                "1" => return Some(28591),
                "2" => return Some(28592),
                "3" => return Some(28593),
                "4" => return Some(28594),
                "5" => return Some(28595),
                "6" => return Some(28596),
                "7" => return Some(28597),
                "9" => return Some(28599),
                "13" => return Some(28603),
                "15" => return Some(28605),
                _ => {}
            }
            if let Some(encoding) = trim_encoding_prefix(encoding, "8") {
                return Some(if encoding == "i" || encoding == "I" {
                    38598
                } else {
                    28598
                });
            } else {
                return None;
            }
        }
        return None;
    }

    const SPECIAL_PARTS: [(&[&str], u32); 14] = [
        (&["hz", "gb", "2312"], 52936),
        (&["big", "5"], 950),
        (&["johab"], 1361),
        (&["koi8", "r"], 20866),
        (&["koi8", "u"], 21866),
        (&["ks", "c", "5601", "1987"], 949),
        (&["macintosh"], 10000),
        (&["shift", "jis"], 932),
        (&["unicode", "fffe"], 1201),
        (&["us", "ascii"], 20127),
        (&["x", "chinese", "eten"], 20002),
        (&["x", "chinese", "cns"], 20000),
        (&["x", "EBCDIC", "KoreanExtended"], 20833),
        (&["x", "Europa"], 29001),
    ];
    if let Some(codepage) = SPECIAL_PARTS
        .iter()
        .find(|(parts, _)| match_encoding_parts(encoding, parts) == Some(""))
        .map(|(_, codepage)| *codepage)
    {
        return Some(codepage);
    }

    if let Some(encoding) = trim_encoding_prefix(encoding, "x") {
        let (parts, encoding) = if let Some(encoding) = trim_encoding_prefix(encoding, "ia5") {
            const X_IA5_PARTS: [(&str, u32); 4] = [
                ("", 20105),
                ("german", 20106),
                ("norwegian", 20108),
                ("swedish", 20107),
            ];
            (&X_IA5_PARTS[..], encoding)
        } else if let Some(encoding) = trim_encoding_prefix(encoding, "iscii") {
            const X_ISCII_PARTS: [(&str, u32); 10] = [
                ("as", 57006),
                ("be", 57003),
                ("de", 57002),
                ("gu", 57010),
                ("ka", 57008),
                ("ma", 57009),
                ("or", 57007),
                ("pa", 57011),
                ("ta", 57004),
                ("te", 57005),
            ];
            (&X_ISCII_PARTS[..], encoding)
        } else if let Some(encoding) = trim_encoding_prefix(encoding, "mac") {
            const X_MAC_PARTS: [(&str, u32); 15] = [
                ("arabic", 10004),
                ("ce", 10029),
                ("chinesesimp", 10008),
                ("chinesetrad", 10002),
                ("croatian", 10082),
                ("cyrillic", 10007),
                ("greek", 10006),
                ("hebrew", 10005),
                ("icelandic", 10079),
                ("japanese", 10001),
                ("korean", 10003),
                ("romanian", 10010),
                ("thai", 10021),
                ("turkish", 10081),
                ("ukrainian", 10017),
            ];
            (&X_MAC_PARTS[..], encoding)
        } else {
            (&[][..], encoding)
        };
        if let Some(codepage) = parts
            .iter()
            .find(|(part, _)| match_encoding_parts(encoding, &[*part]) == Some(""))
            .map(|(_, codepage)| *codepage)
        {
            return Some(codepage);
        }
        return None;
    }

    None
}

pub(super) fn is_no_flag_codepage(codepage: u32) -> bool {
    const NO_FLAG_CODEPAGES: [u32; 8] = [50220, 50221, 50222, 50225, 50227, 50229, 65000, 42];
    const NO_FLAG_RANGES: [RangeInclusive<u32>; 1] = [57002..=57011];
    NO_FLAG_CODEPAGES.contains(&codepage)
        || NO_FLAG_RANGES.iter().any(|range| range.contains(&codepage))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encoding_to_codepage_separator() {
        let testcases = [(65001, &["utf8", "UTF-8", "uTF_8"][..])];
        for (expected, inputs) in testcases {
            for input in inputs {
                assert_eq!(
                    encoding_to_codepage(input),
                    Some(expected),
                    "{expected} {input}"
                );
            }
        }
    }

    #[test]
    fn test_encoding_to_codepage_all_official_names() {
        // https://learn.microsoft.com/en-us/windows/win32/intl/code-page-identifiers
        let testcases = [
            (52936, "hz-gb-2312"),
            (708, "ASMO-708"),
            (950, "big5"),
            (21025, "cp1025"),
            (866, "cp866"),
            (875, "cp875"),
            (50221, "csISO2022JP"),
            (720, "DOS-720"),
            (862, "DOS-862"),
            (51936, "EUC-CN"),
            (20932, "EUC-JP"),
            (51932, "euc-jp"),
            (51949, "euc-kr"),
            (54936, "GB18030"),
            (936, "gb2312"),
            (936, "gbk"),
            (52936, "hz-gb-2312"),
            (858, "IBM00858"),
            (20924, "IBM00924"),
            (1047, "IBM01047"),
            (1140, "IBM01140"),
            (1141, "IBM01141"),
            (1142, "IBM01142"),
            (1143, "IBM01143"),
            (1144, "IBM01144"),
            (1145, "IBM01145"),
            (1146, "IBM01146"),
            (1147, "IBM01147"),
            (1148, "IBM01148"),
            (1149, "IBM01149"),
            (37, "IBM037"),
            (1026, "IBM1026"),
            (20273, "IBM273"),
            (20277, "IBM277"),
            (20278, "IBM278"),
            (20280, "IBM280"),
            (20284, "IBM284"),
            (20285, "IBM285"),
            (20290, "IBM290"),
            (20297, "IBM297"),
            (20420, "IBM420"),
            (20423, "IBM423"),
            (20424, "IBM424"),
            (437, "IBM437"),
            (500, "IBM500"),
            (737, "ibm737"),
            (775, "ibm775"),
            (850, "ibm850"),
            (852, "ibm852"),
            (855, "IBM855"),
            (857, "ibm857"),
            (860, "IBM860"),
            (861, "ibm861"),
            (863, "IBM863"),
            (864, "IBM864"),
            (865, "IBM865"),
            (869, "ibm869"),
            (870, "IBM870"),
            (20871, "IBM871"),
            (20880, "IBM880"),
            (20905, "IBM905"),
            (20838, "IBM-Thai"),
            (50220, "iso-2022-jp"),
            // (50222, "iso-2022-jp"),
            (50225, "iso-2022-kr"),
            (28591, "iso-8859-1"),
            (28603, "iso-8859-13"),
            (28605, "iso-8859-15"),
            (28592, "iso-8859-2"),
            (28593, "iso-8859-3"),
            (28594, "iso-8859-4"),
            (28595, "iso-8859-5"),
            (28596, "iso-8859-6"),
            (28597, "iso-8859-7"),
            (28598, "iso-8859-8"),
            (38598, "iso-8859-8-i"),
            (28599, "iso-8859-9"),
            (1361, "Johab"),
            (20866, "koi8-r"),
            (21866, "koi8-u"),
            (949, "ks_c_5601-1987"),
            (10000, "macintosh"),
            (932, "shift_jis"),
            (1201, "unicodeFFFE"),
            (20127, "us-ascii"),
            (1200, "utf-16"),
            (12000, "utf-32"),
            (12001, "utf-32BE"),
            (65000, "utf-7"),
            (65001, "utf-8"),
            (1250, "windows-1250"),
            (1251, "windows-1251"),
            (1252, "windows-1252"),
            (1253, "windows-1253"),
            (1254, "windows-1254"),
            (1255, "windows-1255"),
            (1256, "windows-1256"),
            (1257, "windows-1257"),
            (1258, "windows-1258"),
            (874, "windows-874"),
            (20002, "x_Chinese-Eten"),
            (20000, "x-Chinese_CNS"),
            (20001, "x-cp20001"),
            (20003, "x-cp20003"),
            (20004, "x-cp20004"),
            (20005, "x-cp20005"),
            (20261, "x-cp20261"),
            (20269, "x-cp20269"),
            (20936, "x-cp20936"),
            (20949, "x-cp20949"),
            (50227, "x-cp50227"),
            (20833, "x-EBCDIC-KoreanExtended"),
            (29001, "x-Europa"),
            (20105, "x-IA5"),
            (20106, "x-IA5-German"),
            (20108, "x-IA5-Norwegian"),
            (20107, "x-IA5-Swedish"),
            (57006, "x-iscii-as"),
            (57003, "x-iscii-be"),
            (57002, "x-iscii-de"),
            (57010, "x-iscii-gu"),
            (57008, "x-iscii-ka"),
            (57009, "x-iscii-ma"),
            (57007, "x-iscii-or"),
            (57011, "x-iscii-pa"),
            (57004, "x-iscii-ta"),
            (57005, "x-iscii-te"),
            (10004, "x-mac-arabic"),
            (10029, "x-mac-ce"),
            (10008, "x-mac-chinesesimp"),
            (10002, "x-mac-chinesetrad"),
            (10082, "x-mac-croatian"),
            (10007, "x-mac-cyrillic"),
            (10006, "x-mac-greek"),
            (10005, "x-mac-hebrew"),
            (10079, "x-mac-icelandic"),
            (10001, "x-mac-japanese"),
            (10003, "x-mac-korean"),
            (10010, "x-mac-romanian"),
            (10021, "x-mac-thai"),
            (10081, "x-mac-turkish"),
            (10017, "x-mac-ukrainian"),
        ];
        for (expected, input) in testcases {
            assert_eq!(encoding_to_codepage(input), Some(expected), "{input}");
        }
    }

    #[test]
    fn test_encoding_to_codepage_invalid() {
        assert_eq!(encoding_to_codepage("invalid_encoding"), None);
    }
}
