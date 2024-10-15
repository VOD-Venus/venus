use crate::{
    components::{dark_mode_btn::DarkMode, home_page::subscripiton::Subscription, title::Title},
    GlobalUI,
};
use leptos::*;

#[derive(Debug, Clone, Copy)]
struct HomeTab<'a> {
    pub id: &'a str,
    pub name: &'a str,
}

/// 首页
#[component]
pub fn Home() -> impl IntoView {
    let home_tabs: [HomeTab; 2] = [
        HomeTab {
            id: "subscription",
            name: "Subscription",
        },
        HomeTab {
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
                                    class=("tab-active", move || ui.tabs.get().home == tab.id)
                                    on:click=move |_| {
                                        if ui.tabs.get().home != tab.id {
                                            ui.tabs.update(|t| t.home = tab.id)
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

            {move || match ui.tabs.get().home {
                "subscription" => view! { <Subscription /> }.into_view(),
                "nodes" => view! { <div>nodes</div> }.into_view(),
                _ => view! { <div>Error: wrong tab id</div> }.into_view(),
            }}

            <DarkMode />
        </div>
    }
}
