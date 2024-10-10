use crate::components::{dark_mode_btn::DarkMode, title::Title};
use leptos::*;

#[derive(Debug, Clone, Copy)]
struct Tabs<'a> {
    /// 首页的 Tab 标签页 ID
    pub home: &'a str,
}

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
    let (index, set_index) = create_signal(Tabs {
        home: "subscription",
    });

    view! {
        <div>
            <Title>Proxies</Title>

            <div>
                <div role="tablist" class="tabs tabs-bordered">
                    <For
                        each=move || home_tabs
                        key=|tab| tab.id
                        children=move |tab| {
                            view! {
                                <a
                                    id=tab.id
                                    role="tab"
                                    class="tab"
                                    class=("tab-active", move || index().home == tab.id)
                                    on:click=move |_| { set_index.update(|t| t.home = tab.id) }
                                >
                                    {tab.name}
                                </a>
                            }
                        }
                    />
                </div>
                <DarkMode />
            </div>
        </div>
    }
}
