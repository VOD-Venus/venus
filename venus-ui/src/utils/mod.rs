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
