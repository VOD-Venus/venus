use leptos::prelude::*;

use crate::components::{dark_mode_btn::DarkMode, title::Title};

#[component]
pub fn Settings() -> impl IntoView {
    view! {
        <div>
            <Title>Settings</Title>

            <div>
                <DarkMode />
            </div>
        </div>
    }
}
