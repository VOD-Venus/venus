use leptos::prelude::*;

use crate::components::subscription_card::SubscriptionCard;

/// 首页中的订阅选项卡
#[component]
pub fn Subscription() -> impl IntoView {
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
                    <SubscriptionCard />

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
