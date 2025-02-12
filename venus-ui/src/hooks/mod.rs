use leptos::prelude::{use_context, RwSignal};

use crate::{GlobalUI, User};

pub fn use_global_ui() -> GlobalUI {
    use_context::<GlobalUI>().expect("GlobalUI state is not set")
}

pub fn use_global_user() -> RwSignal<User> {
    use_context::<GlobalUI>()
        .expect("GlobalUI state is not set")
        .user
}
