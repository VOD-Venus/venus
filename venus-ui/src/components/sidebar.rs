use leptos::prelude::*;
use leptos_router::hooks::use_location;

use crate::hooks::use_global_ui;

/// Navigation item
struct Navi {
    pub name: &'static str,
    pub path: &'static str,
    pub icon: &'static str,
}
/// Navigation items
const NAVI: [Navi; 5] = [
    Navi {
        name: "Proxies",
        path: "/home",
        icon: "icon-[solar--server-2-bold-duotone]",
    },
    Navi {
        name: "Settings",
        path: "/settings",
        icon: "icon-[solar--settings-minimalistic-bold-duotone]",
    },
    Navi {
        name: "Logging",
        path: "/logging",
        icon: "icon-[solar--file-text-bold-duotone]",
    },
    Navi {
        name: "Editor",
        path: "/editor",
        icon: "icon-[solar--code-file-bold-duotone]",
    },
    Navi {
        name: "About",
        path: "/about",
        icon: "icon-[solar--planet-bold-duotone]",
    },
];

/// Sidebar content
///
/// is_mobile: whether the sidebar is in mobile mode
///
/// If is mobile sidebar, the width of the sidebar is 100%, the sidebar will cannot be closed.
#[component]
fn SidebarContent(is_mobile: bool) -> impl IntoView {
    let location = use_location();
    let ui = use_global_ui();

    let children = move |n: Navi| {
        let opened = Memo::new(move |_| {
            if is_mobile {
                true
            } else {
                ui.sidebar_open.get()
            }
        });

        view! {
            <li class="mb-2">
                <a
                    href=n.path
                    class="transition-all duration-300"
                    class=("flex", move || is_mobile)
                >
                    <button
                        class="btn btn-ghost overflow-hidden sm:w-full duration-300"
                        class=("btn-active", move || n.path == location.pathname.get())
                        class=("w-full", move || is_mobile)
                    >
                        <div
                            class="flex items-center justify-start"
                            class=("w-4/6", move || opened.get())
                            class=("w-4", move || !opened.get())
                        >
                            <span
                                class=format!("{} min-w-[1rem] w-4 h-4", n.icon)
                                class=("mr-1", move || opened.get())
                            ></span>
                            <span
                                class="transition-all duration-300 overflow-hidden text-left leading-tight"
                                class=("w-5/6", move || opened.get())
                                class=("w-0", move || !opened.get())
                            >
                                {n.name}
                            </span>
                        </div>
                    </button>
                </a>
            </li>
        }
    };

    view! {
        <>
            // logo
            <div class="flex justify-center w-full" class=("p-2", move || !ui.sidebar_open.get())>
                <img
                    alt="venus"
                    src="public/venus.svg"
                    class="object-contain max-w-[7rem] max-h-28"
                />
            </div>

            // nav
            <div class="flex flex-col justify-between h-full felx-1 items-center">
                <div class="flex flex-col my-4 w-full">
                    <ul class="my-4 bg-transparent rounded-box">
                        <For each=move || NAVI key=|n| n.path children=children />
                    </ul>
                </div>

                // toggle sidebar button
                <span
                    class="icon-[solar--square-double-alt-arrow-left-bold-duotone] transition-all duration-300 hidden sm:block w-7 h-7 cursor-pointer"
                    class=("rotate-180", move || ui.sidebar_open.get())
                    on:click=move |_| ui.sidebar_open.set(!ui.sidebar_open.get())
                ></span>
            </div>
        </>
    }
}

#[component]
pub fn Sidebar() -> impl IntoView {
    let ui = use_global_ui();

    view! {
        <>
            <nav
                class="sm:flex flex-col max-w-xs py-6 bg-gray-100 dark:bg-rua-gray-900 hidden transition-all duration-300"
                class=("w-56", move || ui.sidebar_open.get())
                class=("px-5", move || ui.sidebar_open.get())
                class=("w-20", move || !ui.sidebar_open.get())
                class=("px-3", move || !ui.sidebar_open.get())
            >
                <SidebarContent is_mobile=false />
            </nav>
        </>
    }
}

#[component]
pub fn SidebarMobile() -> impl IntoView {
    view! {
        <>
            <div class="navbar bg-transparent p-0 mt-[-2rem] mx-[-1rem] sm:hidden">
                <div class="flex-none">
                    <div class="z-10 sm:hidden relative">
                        <input id="my-drawer" type="checkbox" class="drawer-toggle" />
                        <label for="my-drawer" class="btn drawer-button btn-square btn-ghost">
                            <span
                                class="icon-[solar--hamburger-menu-line-duotone]"
                                style="width: 24px; height: 24px;"
                            ></span>
                        </label>
                        <div class="drawer-side">
                            <label
                                for="my-drawer"
                                aria-label="close sidebar"
                                class="drawer-overlay"
                            ></label>
                            <ul class="bg-base-200 text-base-content min-h-full w-72 p-4">
                                <SidebarContent is_mobile=true />
                            </ul>
                        </div>
                    </div>
                </div>

                <div class="flex-1">
                    <a class="btn btn-ghost text-xl p-0">Venus</a>
                </div>
            </div>
        </>
    }
}
