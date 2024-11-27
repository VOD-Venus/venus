use rand::Rng;
use std::borrow::Cow;

/// Capitalize the first letter of a string
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

/// Convert an error to a string
pub fn error_to_string(err: impl std::fmt::Display) -> String {
    format!("Error: {}", err)
}

/// Generate a random ID of the given size
pub fn nanoid(size: usize) -> String {
    const CHARSET: &str = "useandom-26T198340PX75pxJACKVERYMINDBUSHWOLF_GQZbfghjklqvwyzrict";
    const MASK: u8 = 63; // 0b00111111 to select 6 bits (index range 0-63)
    let mut rng = rand::thread_rng();

    (0..size)
        .map(|_| {
            let rand_value: u8 = rng.gen();
            CHARSET.as_bytes()[(rand_value & MASK) as usize] as char
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_capitalize_first_empty_string() {
        let input = "";
        let result = capitalize_first(input);
        assert_eq!(result, "", "Expected an empty string for an empty input");
    }

    #[test]
    fn test_capitalize_first_single_lowercase_char() {
        let input = "a";
        let result = capitalize_first(input);
        assert_eq!(result, "A", "Expected 'A' for input 'a'");
    }

    #[test]
    fn test_capitalize_first_single_uppercase_char() {
        let input = "A";
        let result = capitalize_first(input);
        assert_eq!(result, "A", "Expected 'A' for input 'A'");
    }

    #[test]
    fn test_capitalize_first_lowercase_word() {
        let input = "hello";
        let result = capitalize_first(input);
        assert_eq!(result, "Hello", "Expected 'Hello' for input 'hello'");
    }

    #[test]
    fn test_capitalize_first_uppercase_word() {
        let input = "Hello";
        let result = capitalize_first(input);
        assert_eq!(result, "Hello", "Expected 'Hello' for input 'Hello'");
    }

    #[test]
    fn test_capitalize_first_mixed_case() {
        let input = "hELLo";
        let result = capitalize_first(input);
        assert_eq!(result, "HELLo", "Expected 'HELLo' for input 'hELLo'");
    }

    #[test]
    fn test_capitalize_first_non_alphabetic_start() {
        let input = "1hello";
        let result = capitalize_first(input);
        assert_eq!(result, "1hello", "Expected '1hello' for input '1hello'");
    }

    #[test]
    fn test_capitalize_first_unicode() {
        let input = "ábc";
        let result = capitalize_first(input);
        assert_eq!(result, "Ábc", "Expected 'Ábc' for input 'ábc'");
    }

    #[test]
    fn test_capitalize_first_whitespace() {
        let input = " hello";
        let result = capitalize_first(input);
        assert_eq!(result, " hello", "Expected ' hello' for input ' hello'");
    }

    #[test]
    fn test_error_to_string_with_str() {
        let error_message = "Something went wrong";
        let result = error_to_string(error_message);
        assert_eq!(
            result, "Error: Something went wrong",
            "Expected 'Error: Something went wrong'"
        );
    }

    #[test]
    fn test_error_to_string_with_string() {
        let error_message = String::from("A string error occurred");
        let result = error_to_string(error_message);
        assert_eq!(
            result, "Error: A string error occurred",
            "Expected 'Error: A string error occurred'"
        );
    }

    #[test]
    fn test_error_to_string_with_number() {
        let error_code = 404;
        let result = error_to_string(error_code);
        assert_eq!(result, "Error: 404", "Expected 'Error: 404'");
    }

    #[test]
    fn test_error_to_string_with_custom_error() {
        #[derive(Debug)]
        struct CustomError {
            details: String,
        }

        impl std::fmt::Display for CustomError {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "Custom error: {}", self.details)
            }
        }

        let custom_error = CustomError {
            details: String::from("An example custom error"),
        };

        let result = error_to_string(custom_error);
        assert_eq!(
            result, "Error: Custom error: An example custom error",
            "Expected 'Error: Custom error: An example custom error'"
        );
    }

    #[test]
    fn test_nanoid_length() {
        let size = 21;
        let id = nanoid(size);
        assert_eq!(
            id.len(),
            size,
            "Generated ID does not match the expected length"
        );
    }

    #[test]
    fn test_nanoid_charset() {
        const CHARSET: &str = "useandom-26T198340PX75pxJACKVERYMINDBUSHWOLF_GQZbfghjklqvwyzrict";
        let size = 21;
        let id = nanoid(size);

        // Ensure all characters in the ID are part of the CHARSET
        for ch in id.chars() {
            assert!(
                CHARSET.contains(ch),
                "Generated ID contains an invalid character: '{}'",
                ch
            );
        }
    }

    #[test]
    fn test_nanoid_randomness() {
        let size = 21;
        let id1 = nanoid(size);
        let id2 = nanoid(size);
        assert_ne!(id1, id2, "Generated IDs are not unique");
    }
}
