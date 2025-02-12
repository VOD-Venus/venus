use crate::{
    api::{axios, BaseResponse, RequestApi},
    components::subscription_card::{SubCardForm, SubscriptionCard},
    hooks::use_global_user,
    utils::error_to_string,
    User,
};
use gloo::net::http::Method;
use leptos::{logging, prelude::*};
use web_sys::MouseEvent;

async fn add_subscription(subs_form: (SubCardForm, User)) -> Result<BaseResponse<()>, String> {
    let address = format!("{}{}", subs_form.1.server, RequestApi::AddSubscription);
    let resquest = axios(&address, Method::POST)
        .header("Content-Type", "application/json")
        .body(serde_json::to_string(&subs_form.0).map_err(error_to_string)?)
        .map_err(error_to_string)?
        .send()
        .await;
    match resquest {
        Ok(response) => response.json().await.map_err(error_to_string),
        Err(err) => Err(err.to_string()),
    }
}

/// 首页中的订阅选项卡
#[component]
pub fn Subscription() -> impl IntoView {
    let (form, set_form) = signal(SubCardForm {
        name: "".into(),
        url: "".into(),
    });

    let user = use_global_user();

    let form_ref: NodeRef<leptos::html::Form> = NodeRef::new();
    let add_action: Action<SubCardForm, Result<BaseResponse<()>, String>, SyncStorage> =
        Action::new_unsync(move |form: &SubCardForm| add_subscription((form.clone(), user.get())));
    let add_loading = add_action.pending();
    let add_result = add_action.value();
    let handle_submit = move |e: MouseEvent| {
        if add_loading() {
            return;
        }
        e.prevent_default();
        let form_ref = form_ref.get().expect("form element is not rendered");
        let valided = form_ref.check_validity();
        if !valided {
            form_ref.report_validity();
            return;
        }
        add_action.dispatch(form());
    };
    Effect::new(move |_| {
        logging::log!("test {:?}", add_result.get());
    });

    view! {
        <div class="py-4">
            <div class="pb-4">
                // 设置的标题
                <div class="px-4 pb-2 text-sm">
                    <div>Subscription Settings</div>
                </div>
                <div class="p-4 rounded-lg bg-stone-50 dark:bg-rua-gray-800">
                    <button class="mr-2 btn btn-sm" onclick="add_modal.showModal()">
                        Add
                    </button>
                    <SubscriptionCard
                        form=form
                        set_form=set_form
                        on_ok=handle_submit
                        on_close=|_| {}
                        node_ref=form_ref
                        loading=add_loading
                    />

                    <button class="btn btn-sm">Update All</button>
                </div>
            </div>

            <div class="pb-4">
                <div class="px-4 pb-2 text-sm">
                    <div>Subcriptions</div>
                </div>

                <div class="flex flex-wrap">
                    <div class="shadow-xl card dark:bg-base-300 bg-base-100 w-96 mr-4 mb-4">
                        <div class="card-body">
                            <h2 class="card-title">RUA!</h2>
                            <div class="text-gray-400">"https://google.com"</div>
                            <div class="justify-end card-actions">
                                <button class="btn btn-sm">Buy Now</button>
                            </div>
                        </div>
                    </div>
                    <div class="shadow-xl card dark:bg-base-300 bg-base-100 w-96 mr-4 mb-4">
                        <div class="card-body">
                            <h2 class="card-title">RUA!</h2>
                            <div class="text-gray-400">"https://google.com"</div>
                            <div class="justify-end card-actions">
                                <button class="btn btn-sm">Buy Now</button>
                            </div>
                        </div>
                    </div>
                    <div class="shadow-xl card dark:bg-base-300 bg-base-100 w-96 mr-4 mb-4">
                        <div class="card-body">
                            <h2 class="card-title">RUA!</h2>
                            <div class="text-gray-400">"https://google.com"</div>
                            <div class="justify-end card-actions">
                                <button class="btn btn-sm">Buy Now</button>
                            </div>
                        </div>
                    </div>
                    <div class="shadow-xl card dark:bg-base-300 bg-base-100 w-96 mr-4 mb-4">
                        <div class="card-body">
                            <h2 class="card-title">RUA!</h2>
                            <div class="text-gray-400">"https://google.com"</div>
                            <div class="justify-end card-actions">
                                <button class="btn btn-sm">Buy Now</button>
                            </div>
                        </div>
                    </div>
                    <div class="shadow-xl card dark:bg-base-300 bg-base-100 w-96 mr-4 mb-4">
                        <div class="card-body">
                            <h2 class="card-title">RUA!</h2>
                            <div class="text-gray-400">"https://google.com"</div>
                            <div class="justify-end card-actions">
                                <button class="btn btn-sm">Buy Now</button>
                            </div>
                        </div>
                    </div>
                </div>

            </div>
        </div>
    }
}
