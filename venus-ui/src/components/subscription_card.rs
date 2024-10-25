use leptos::*;

/// Subscription modal card
/// use `add_modal.showModal()` on click event
#[component]
pub fn SubscriptionCard() -> impl IntoView {
    view! {
        <dialog id="add_modal" class="modal">
            <div class="modal-box">
                <h3 class="text-lg font-bold">Subscription</h3>
                <p class="py-4">Press ESC key or click the button below to close</p>
                <div class="modal-action">
                    <form method="dialog">
                        <button class="btn">Close</button>
                    </form>
                </div>
            </div>
        </dialog>
    }
}
