use leptos::*;

use crate::GlobalUI;

/// 全局通知栏
#[component]
pub fn Notifications() -> impl IntoView {
    let nts = use_context::<GlobalUI>()
        .expect("GlobalUI state is not set")
        .notifications;

    let icon = move |kind: crate::NotificationKind| match kind {
        crate::NotificationKind::Success => {
            view! { <span class="icon-[solar--check-circle-line-duotone]"></span> }
        }
        crate::NotificationKind::Info => {
            view! { <span class="icon-[solar--info-circle-line-duotone]"></span> }
        }
        crate::NotificationKind::Warning => {
            view! { <span class="icon-[solar--shield-warning-line-duotone]"></span> }
        }
        crate::NotificationKind::Error => {
            view! { <span class="icon-[solar--add-circle-line-duotone] rotate-45"></span> }
        }
    };

    view! {
        <div class="toast toast-top toast-end">
            <For each=move || nts.get() key=|nt| nt.key let:nt>
                <div class="alert shadow-lg bg-white animate-slide-in-right">
                    {icon(nt.kind)} <span>{nt.message}</span>
                </div>
            </For>
        </div>
    }
}
