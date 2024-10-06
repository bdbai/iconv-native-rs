pub(crate) fn trim_encoding_prefix<'i>(ascii_input: &'i str, prefix: &str) -> Option<&'i str> {
    let (input_prefix, input_remaining) = ascii_input.split_at_checked(prefix.len())?;
    if !prefix.eq_ignore_ascii_case(input_prefix) {
        return None;
    }
    Some(
        input_remaining
            .strip_prefix(&['-', '_', ' '][..])
            .unwrap_or(input_remaining),
    )
}

#[allow(dead_code)]
pub(crate) fn match_encoding_parts<'i>(input: &'i str, parts: &[&str]) -> Option<&'i str> {
    parts
        .iter()
        .try_fold(input, |input, &part| trim_encoding_prefix(input, part))
}

#[allow(dead_code)]
pub(crate) fn match_encoding_parts_exact(input: &str, parts: &[&str]) -> bool {
    match_encoding_parts(input, parts) == Some("")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_trim_encoding_prefix() {
        let testcases = [
            ("utf8", "UTf", Some("8")),
            ("uTf-8", "utf", Some("8")),
            ("UTF_8", "utf", Some("8")),
            ("UTf 8", "utf", Some("8")),
            ("utf", "utf", Some("")),
            ("ut", "utf", None),
        ];

        for (input, prefix, expected) in testcases {
            assert_eq!(
                trim_encoding_prefix(input, prefix),
                expected,
                "{input}, {prefix}, {expected:?}"
            );
        }
    }

    #[test]
    fn test_match_encoding_parts() {
        let testcases = [
            ("utf8", &["utf"][..], Some("8")),
            ("uTf-8", &["utf", "8"], Some("")),
            ("utf_7", &["utf", "8"], None),
            ("utf-16be", &["utf"], Some("16be")),
            ("utf-16be", &["utf", "16"], Some("be")),
            ("utf-16be", &["utf", "16", "be"], Some("")),
            ("utf-16be", &["utf", "16", "le"], None),
            ("utf-16be", &["utf", "8"], None),
            ("utf-16-BE", &["utf", "16", "be"], Some("")),
        ];
        for (input, parts, expected) in testcases {
            assert_eq!(
                match_encoding_parts(input, parts),
                expected,
                "{input}, {parts:?}, {expected:?}"
            );
        }
    }

    #[test]
    fn test_match_encoding_parts_exact() {
        let testcases = [
            ("utf8", &["utf"][..], false),
            ("uTf-8", &["utf", "8"], true),
            ("utf_7", &["utf", "8"], false),
            ("utf-16be", &["utf"], false),
            ("utf-16be", &["utf", "16"], false),
            ("utf-16be", &["utf", "16", "be"], true),
            ("utf-16be", &["utf", "16", "le"], false),
            ("utf-16be", &["utf", "8"], false),
            ("utf-16-BE", &["utf", "16", "be"], true),
        ];
        for (input, parts, expected) in testcases {
            assert_eq!(
                match_encoding_parts_exact(input, parts),
                expected,
                "{input}, {parts:?}, {expected}"
            );
        }
    }
}
