use std::borrow::Cow;

#[macro_export]
macro_rules! clsx {
    // Match a single item
    ($item:expr) => {
        {
            let mut result = String::new();
            let val = format!("{}", &$item);
            result.push_str(&val);
            result
        }
    };

    // Match multiple items
    ($($item:expr),+) => {
        {
            let mut result = String::new();
            let mut first = true;
            $(
                // if let Some(val) = to_val(&$item) {
                    let val = format!("{}", &$item);
                    if !first {
                        result.push(' ');
                    }
                    result.push_str(&val);
                    #[allow(unused_assignments)]
                    first = false;
                // }
            )+
            result
        }
    };
}

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
