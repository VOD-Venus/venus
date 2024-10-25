use leptos::*;

/// Subscription modal card
/// use `add_modal.showModal()` on click event
#[component]
pub fn SubscriptionCard() -> impl IntoView {
    view! {
        <dialog id="add_modal" class="modal">
            <div class="modal-box">
                <h3 class="text-lg font-bold">Subscription</h3>

                <div class="py-4 flex flex-col gap-4">
                    <label class="input input-bordered flex items-center gap-2">
                        Name <input type="text" class="grow" placeholder="rua" />
                    </label>
                    <label class="input input-bordered flex items-center gap-2">
                        URL <input type="text" class="grow" placeholder="https://rua.rua" />
                    </label>
                </div>

                <div class="modal-action">
                    <button class="btn btn-primary">Confirm</button>
                    <form method="dialog">
                        <button class="btn">Close</button>
                    </form>
                </div>
            </div>
        </dialog>
    }
}
