use leptos::prelude::use_context;

use crate::GlobalUI;

pub fn use_global_ui() -> GlobalUI {
    use_context::<GlobalUI>().expect("GlobalUI state is not set")
}
