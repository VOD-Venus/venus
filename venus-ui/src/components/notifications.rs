use leptos::*;

use crate::GlobalUI;
use gloo::timers::callback::Timeout;

const MAX_NOTIFICATIONS: usize = 5;

/// 全局通知栏
#[component]
pub fn Notifications() -> impl IntoView {
    let nts = use_context::<GlobalUI>()
        .expect("GlobalUI state is not set")
        .notifications;

    view! {
        <div class="toast toast-top toast-end">
            <For each=move || nts.get() key=|nt| nt.key.clone() let:nt>
                <Notification kind=nt.kind message=nt.message />
            </For>
        </div>
    }
}

const NOTIFICATION_TIMEOUT: u32 = 5_000;

/// 通知消息
///
/// ## Arguments
/// * `kind` - 通知类型
/// * `message` - 通知消息
///
/// 当通知显示了后，将会在 `NOTIFICATION_TIMEOUT` 毫秒后自动消失。
/// 并修改 `nts` 的状态，将其中的通知删除。
#[component]
pub fn Notification(kind: crate::NotificationKind, message: String) -> impl IntoView {
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

    let nts = use_context::<GlobalUI>()
        .expect("GlobalUI state is not set")
        .notifications;

    let (need_move, set_need_move) = create_signal(false);
    let timeout = Timeout::new(NOTIFICATION_TIMEOUT, move || {
        set_need_move.set(true);
    });
    timeout.forget();

    let timeout = Timeout::new(NOTIFICATION_TIMEOUT + 299, move || {
        nts.update(|nts| {
            // shorten the notifications from head
            nts.drain(..1);
        });
    });
    timeout.forget();

    view! {
        <div class=move || {
            if need_move.get() {
                "alert shadow-lg bg-white animate-slide-out-right-slow"
            } else {
                "alert shadow-lg bg-white animate-slide-in-right"
            }
        }>{icon(kind)} <span>{message}</span></div>
    }
}
