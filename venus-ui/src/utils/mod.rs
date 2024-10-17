use std::borrow::Cow;

pub fn capitalize_first(s: &str) -> Cow<str> {
    if s.is_empty() {
        return s.into();
    }

    let mut chars = s.chars();

    let first_char = chars
        .next()
        .map(|c| c.to_uppercase().to_string())
        .unwrap_or_default();

    let rest: String = chars.collect();

    format!("{}{}", first_char, rest).into()
}
