use crate::{
    components::{dark_mode_btn::DarkMode, home_page::subscripiton::Subscription, title::Title},
    GlobalUI,
};
use leptos::prelude::*;

#[derive(Debug, Clone, Copy)]
struct ProxyTab<'a> {
    pub id: &'a str,
    pub name: &'a str,
}

/// 首页
#[component]
pub fn Proxies() -> impl IntoView {
    let home_tabs: [ProxyTab; 2] = [
        ProxyTab {
            id: "subscription",
            name: "Subscription",
        },
        ProxyTab {
            id: "nodes",
            name: "Nodes",
        },
    ];

    let ui = use_context::<GlobalUI>().expect("GlobalUI state is not set");

    view! {
        <div>
            <Title>Proxies</Title>

            <div class="flex">
                <div role="tablist" class="tabs tabs-bordered">
                    <For
                        each=move || home_tabs
                        key=|tab| tab.id
                        children=move |tab| {
                            view! {
                                <a
                                    id=tab.id
                                    role="tab"
                                    class="tab transition-all duration-300"
                                    class=("tab-active", move || ui.tabs.get().proxies == tab.id)
                                    on:click=move |_| {
                                        if ui.tabs.get().proxies != tab.id {
                                            ui.tabs.update(|t| t.proxies = tab.id)
                                        }
                                    }
                                >
                                    {tab.name}
                                </a>
                            }
                        }
                    />
                </div>
            </div>

            {move || match ui.tabs.get().proxies {
                "subscription" => view! { <Subscription /> }.into_any(),
                "nodes" => view! { <div>nodes</div> }.into_any(),
                _ => view! { <div>Error: wrong tab id</div> }.into_any(),
            }}

            <DarkMode />
        </div>
    }
}
