use leptos::*;

/// 首页中的订阅选项卡
#[component]
pub fn Subscription() -> impl IntoView {
    view! {
        <div class="py-4">
            // 设置的标题
            <div class="px-4 pb-2 text-sm">
                <div>Subscription Settings</div>
            </div>
            <div class="p-4 rounded-lg bg-slate-50">
                <button class="mr-2 btn btn-sm">Add</button>
                <button class="btn btn-sm">Update All</button>
            </div>
        </div>

        <div class="py-4">
            <div class="px-4 text-sm">
                <div>Subcriptions</div>
            </div>
            <div class="shadow-xl card dark:bg-base-300 bg-base-100 w-96">
                <div class="card-body">
                    <h2 class="card-title">RUA!</h2>
                    <div class="text-gray-400">"https://google.com"</div>
                    <div class="justify-end card-actions">
                        <button class="btn btn-sm">Buy Now</button>
                    </div>
                </div>
            </div>
        </div>
    }
}
