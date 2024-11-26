use leptos::*;

use crate::GlobalUI;

/// 全局通知栏
#[component]
pub fn Notifications() -> impl IntoView {
    let nts = use_context::<GlobalUI>()
        .expect("GlobalUI state is not set")
        .notifications;

    view! {
        <div class="toast toast-top toast-end">
            <For each=move || nts.get() key=|nt| nt.key let:nt>
                <div class="alert shadow-lg bg-white animate-slide-in-right">
                    <span>{nt.message}</span>
                </div>
            </For>
        </div>
    }
}
