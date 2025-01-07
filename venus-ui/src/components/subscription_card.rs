use leptos::ev::Event;
use leptos::prelude::*;
use serde::{Deserialize, Serialize};
use web_sys::MouseEvent;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubCardForm {
    pub name: String,
    pub url: String,
}

/// Subscription modal card
/// use `add_modal.showModal()` on click event to show
#[component]
pub fn SubscriptionCard(
    form: ReadSignal<SubCardForm>,
    set_form: WriteSignal<SubCardForm>,
    /// Click the confirm button
    on_ok: impl FnMut(MouseEvent) + 'static,
    /// Click the close button
    on_close: impl FnMut(MouseEvent) + 'static,
) -> impl IntoView {
    enum FormTarget {
        Name,
        Url,
    }
    let handle_change = move |target: FormTarget| {
        use FormTarget::*;
        move |e: Event| {
            let value = event_target_value(&e);
            match target {
                Name => set_form.update(|f| f.name = value),
                Url => set_form.update(|f| f.url = value),
            }
        }
    };

    view! {
        <dialog id="add_modal" class="modal">
            <div class="modal-box">
                <h3 class="text-lg font-bold">Subscription</h3>

                <div class="py-4 flex flex-col gap-4">
                    <label class="input input-bordered flex items-center gap-2">
                        Name
                        <input
                            type="text"
                            class="grow"
                            placeholder="rua"
                            prop:value=move || form().name
                            on:change=handle_change(FormTarget::Name)
                        />
                    </label>
                    <label class="input input-bordered flex items-center gap-2">
                        URL
                        <input
                            type="text"
                            class="grow"
                            placeholder="https://rua.rua"
                            prop:value=move || form().url
                            on:change=handle_change(FormTarget::Url)
                        />
                    </label>
                </div>

                <div class="modal-action">
                    <button class="btn btn-primary" on:click=on_ok>
                        Confirm
                    </button>
                    <form method="dialog">
                        <button class="btn" on:click=on_close>
                            Close
                        </button>
                    </form>
                </div>
            </div>
        </dialog>
    }
}
