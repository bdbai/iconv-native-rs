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

pub(crate) fn match_encoding_parts<'i>(input: &'i str, parts: &[&str]) -> Option<&'i str> {
    let mut input = input;
    for part in parts {
        if let Some(remaining) = trim_encoding_prefix(input, part) {
            input = remaining;
        } else {
            return None;
        }
    }
    Some(input)
}

pub(crate) fn match_encoding_parts_exact(input: &str, parts: &[&str]) -> bool {
    match_encoding_parts(input, parts) == Some("")
}

pub(crate) fn is_encoding_byte_order_ambiguous(encoding: &str) -> bool {
    let Some(rem) = trim_encoding_prefix(encoding, "utf") else {
        return false;
    };
    let rem = if let Some(rem) = trim_encoding_prefix(rem, "16") {
        rem
    } else if let Some(rem) = trim_encoding_prefix(rem, "32") {
        rem
    } else {
        return false;
    };
    trim_encoding_prefix(rem, "be")
        .or_else(|| trim_encoding_prefix(rem, "le"))
        .is_none()
}
