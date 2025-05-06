use leptos::{
    prelude::{use_context, RwSignal},
    view,
};
use thaw::{Toast, ToastBody, ToastIntent, ToastOptions, ToastTitle, ToasterInjection};

use crate::{GlobalUI, User};

pub fn use_global_ui() -> GlobalUI {
    use_context::<GlobalUI>().expect("GlobalUI state is not set")
}

pub fn use_global_user() -> RwSignal<User> {
    use_context::<GlobalUI>()
        .expect("GlobalUI state is not set")
        .user
}

pub fn dispatch_toast(toaster: ToasterInjection, intent: ToastIntent, title: String, body: String) {
    toaster.dispatch_toast(
        move || {
            view! {
                <Toast>
                    <ToastTitle>{title}</ToastTitle>
                    <ToastBody>{body}</ToastBody>
                </Toast>
            }
        },
        ToastOptions::default().with_intent(intent),
    );
}
