use std::borrow::Cow;

use crate::{
    api::{axios, BaseResponse, RequestApi},
    components::subscription_card::{SubCardForm, SubscriptionCard},
    hooks::{use_global_ui, use_global_user},
    utils::error_to_string,
    User,
};
use gloo::net::http::Method;
use leptos::prelude::*;
use serde::{Deserialize, Serialize};
use web_sys::MouseEvent;

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(rename_all = "camelCase")]
/// Core subscriptions
pub struct Subscription {
    pub name: Cow<'static, str>,
    pub url: Cow<'static, str>,
    pub nodes: Vec<Node>,
}
/// Subscription nodes
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Node {
    pub v: Cow<'static, str>,
    // Node name
    pub ps: Cow<'static, str>,
    // Address
    pub add: Cow<'static, str>,
    pub port: Cow<'static, str>,
    pub id: Cow<'static, str>,
    // AlertID
    pub aid: Cow<'static, str>,
    // Protocol type determine streamSettings network field
    pub net: Cow<'static, str>,
    // Protocol type
    #[serde(rename = "type")]
    pub type_field: Cow<'static, str>,
    pub host: Cow<'static, str>,
    // streamSettings
    pub path: Cow<'static, str>,
    // Determine streamSettings security field
    pub tls: Cow<'static, str>,
    // Determine streamSettings headers sni
    pub sni: Cow<'static, str>,
    pub alpn: Cow<'static, str>,
    // Add by manually
    // The subscription group
    pub subs: Option<Cow<'static, str>>,
    // Current node speed, upload and download
    pub speed: Option<f64>,
    // Current node delay
    pub delay: Option<u64>,
    // Node connectivity
    pub connectivity: Option<bool>,
    // Node unique ID
    pub node_id: Option<Cow<'static, str>>,
    // Node raw link from subcription link
    pub raw_link: Option<Cow<'static, str>>,
    // Node net type
    pub node_type: Option<NodeType>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum NodeType {
    Vmess,
    Vless,
    SS,
    SSR,
    Trojan,
    Trojango,
    HttpProxy,
    HttpsProxy,
    SOCKS5,
    HTTP2,
    Unknown,
}
impl From<&str> for NodeType {
    fn from(value: &str) -> Self {
        use NodeType::*;
        match value.to_lowercase().as_str() {
            "vmess" => Vmess,
            "vless" => Vless,
            "ss" => SS,
            "ssr" => SSR,
            "trojan" => Trojan,
            "trojan-go" => Trojango,
            "http-proxy" => HttpProxy,
            "https-proxy" => HttpsProxy,
            "socks5" => SOCKS5,
            "http2" => HTTP2,
            _ => Unknown,
        }
    }
}
impl NodeType {
    pub fn as_str(&self) -> &str {
        use NodeType::*;
        match self {
            Vmess => "vmess",
            Vless => "vless",
            SS => "ss",
            SSR => "ssr",
            Trojan => "trojan",
            Trojango => "trojan-go",
            HttpProxy => "http-proxy",
            HttpsProxy => "https-proxy",
            SOCKS5 => "socks5",
            HTTP2 => "http2",
            Unknown => "unknown",
        }
    }
}

/// 获取订阅列表
///
/// ## Arguments
///
/// * `user` - 用户信息
pub async fn get_subscriptions(server: String) -> Result<BaseResponse<Vec<Subscription>>, String> {
    let address = format!("{}{}", server, RequestApi::ListSubscriptions);
    let resquest = axios(&address, Method::GET)
        .header("Content-Type", "application/json")
        .send()
        .await;
    match resquest {
        Ok(response) => response.json().await.map_err(error_to_string),
        Err(err) => Err(err.to_string()),
    }
}

/// 添加订阅
///
/// ## Arguments
///
/// * `subs_form` - 订阅表单
/// * `user` - 用户信息
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

    let ui = use_global_ui();
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
        let result = add_result.get();
        if let Some(res) = result {
            match res {
                Ok(response) => {
                    // ui.notifications.update(|nts| {
                    //     if response.code == 200 {
                    //         // nts.push(Notification::new(
                    //         //     NotificationKind::Success,
                    //         //     "Add subscription success".into(),
                    //         // ));
                    //     } else {
                    //         // nts.push(Notification::new(
                    //         //     NotificationKind::Success,
                    //         //     response.message.clone(),
                    //         // ));
                    //     }
                    // }),
                }
                Err(err) => {
                    // ui.notifications.update(|nts| {
                    //     nts.push(Notification::new(
                    //         NotificationKind::Error,
                    //         format!("Add subscription failed {}", err),
                    //     ));
                    // });
                }
            }
        }
    });

    // subscriptions
    let subscriptions = move || ui.proxies.get().subscriptions;

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
                    <For
                        each=move || subscriptions()
                        key=|sub| sub.url.clone()
                        children=move |sub| {
                            view! {
                                <div class="shadow-xl card dark:bg-base-300 bg-base-100 w-96 mr-4 mb-4">
                                    <div class="card-body">
                                        <h2 class="card-title">{sub.name.clone()}</h2>
                                        <div class="text-gray-400">{sub.url.clone()}</div>
                                        <div class="justify-end card-actions">
                                            <button class="btn btn-sm">Buy Now</button>
                                        </div>
                                    </div>
                                </div>
                            }
                        }
                    />
                </div>
            </div>
        </div>
    }
}
