use leptos::web_sys::MouseEvent;
use leptos::{ev::Event, logging, prelude::*};
use leptos_router::hooks::use_navigate;
use serde::{Deserialize, Serialize};

use crate::api::{BaseResponse, RequestApi};
use crate::hooks::use_global_ui;
use crate::User;
use crate::{utils::error_to_string, Notification, NotificationKind};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Data {
    pub access_token: String,
    pub token_type: String,
}

/// 登录接口返回的数据
type LoginResponse = BaseResponse<Data>;

/// 登录接口需要的参数
#[derive(Debug, Clone, Serialize, Deserialize)]
struct LoginBody {
    pub username: String,
    pub password: String,
}

/// Login to the server
///
/// ## Arguments
/// * `login_form` - The login form
async fn login(login_form: LoginForm) -> Result<LoginResponse, String> {
    use crate::api::post;

    let address = format!("{}{}", login_form.server, RequestApi::Login);
    let login_body = LoginBody {
        username: login_form.username,
        password: login_form.password,
    };
    post(
        &address,
        serde_json::to_string(&login_body).map_err(error_to_string)?,
    )
    .await
}
/// 登录用的表单
#[derive(Debug, Clone, Serialize, Deserialize)]
struct LoginForm {
    pub server: String,
    pub username: String,
    pub password: String,
}

#[component]
pub fn Login() -> impl IntoView {
    let (form, set_form) = signal(LoginForm {
        server: "http://localhost:4001".into(),
        username: "".into(),
        password: "".into(),
    });

    /// The form input field corresponding to the field
    enum FormTarget {
        Server,
        Username,
        Password,
    }
    let handle_change = move |target: FormTarget| {
        use FormTarget::*;
        move |e: Event| {
            let value = event_target_value(&e);
            match target {
                Server => set_form.update(|f| f.server = value),
                Username => set_form.update(|f| f.username = value),
                Password => set_form.update(|f| f.password = value),
            }
        }
    };

    let state = use_global_ui();
    let nts = state.notifications;
    let form_ref: NodeRef<leptos::html::Form> = NodeRef::new();

    let navigate = use_navigate();
    // 如果用户已经登录，则跳转到首页
    Effect::new(move |_| {
        if !state.user.read().token.is_empty() {
            navigate("/dashboard", Default::default());
        }
    });

    // 登录方法 点击登录按钮后触发
    let login_action: Action<LoginForm, Result<BaseResponse<Data>, String>, SyncStorage> =
        Action::new_unsync(|login_form: &LoginForm| login(login_form.clone()));
    let login_loading = login_action.pending();
    let login_result = login_action.value();
    let navigate = use_navigate();
    // after submit
    Effect::new(move |_| {
        let login_handler = || -> anyhow::Result<()> {
            let login_result = login_result.read();
            let result = login_result
                .as_ref()
                .ok_or(anyhow::anyhow!("login result is none"))?;
            match result {
                Ok(response) => {
                    if let Some(data) = &response.data {
                        nts.update(|nts| {
                            nts.push(Notification::new(
                                NotificationKind::Success,
                                "Login success".into(),
                            ));
                        });
                        let user = User {
                            server: form().server.clone(),
                            username: form().username.clone(),
                            token: data.access_token.clone(),
                            token_type: data.token_type.clone(),
                        };
                        state.user.set(user);
                        navigate("/home", Default::default());
                    } else {
                        nts.update(|nts| {
                            nts.push(Notification::new(
                                NotificationKind::Error,
                                response.message.clone(),
                            ));
                        });
                    }
                }
                Err(err) => {
                    logging::error!("login error {:?}", err);
                    nts.update(|nts| {
                        nts.push(Notification::new(
                            NotificationKind::Error,
                            "Login failed".into(),
                        ));
                    });
                }
            }
            Ok(())
        };
        login_handler().ok();
    });
    let handle_submit = move |e: MouseEvent| {
        if login_loading() {
            return;
        }
        e.prevent_default();
        let form_ref = form_ref.get().expect("form element is not rendered");
        let valided = form_ref.check_validity();
        if !valided {
            form_ref.report_validity();
            return;
        }
        login_action.dispatch(form());
    };

    view! {
        <div class="hero bg-base-200 min-h-screen">
            <div class="hero-content flex-col lg:flex-row-reverse">
                <div class="text-center lg:text-left">
                    <h1 class="text-5xl font-bold">Login now!</h1>
                    <p class="py-6">
                        Provident cupiditate voluptatem et in. Quaerat fugiat ut assumenda excepturi exercitationem
                        quasi. In deleniti eaque aut repudiandae et a id nisi.
                    </p>
                </div>
                <div class="card bg-base-100 w-full max-w-sm shrink-0 shadow-2xl">
                    <form class="card-body" node_ref=form_ref>
                        <div class="form-control">
                            <span class="label-text mb-2">Server</span>
                            <label class="input input-bordered flex items-center gap-2">
                                <span class="icon-[solar--server-2-bold-duotone]"></span>
                                <input
                                    type="text"
                                    class="grow"
                                    prop:value=move || form().server
                                    placeholder="Server"
                                    required
                                    on:change=handle_change(FormTarget::Server)
                                />
                            </label>
                        </div>
                        <div class="form-control">
                            <span class="label-text mb-2">Username</span>
                            <label class="input input-bordered flex items-center gap-2">
                                <span class="icon-[solar--user-bold-duotone] h-4 w-4"></span>
                                <input
                                    type="text"
                                    class="grow"
                                    prop:value=move || form().username
                                    placeholder="Username"
                                    required
                                    on:change=handle_change(FormTarget::Username)
                                />
                            </label>
                        </div>
                        <div class="form-control">
                            <span class="label-text mb-2">Password (min 6)</span>
                            <label class="input input-bordered flex items-center gap-2">
                                <span class="icon-[solar--lock-password-bold-duotone]"></span>
                                <input
                                    type="password"
                                    class="grow"
                                    prop:value=move || form().password
                                    placeholder="Password"
                                    required
                                    minlength="6"
                                    pattern="[a-zA-Z0-9]{6,}"
                                    on:change=handle_change(FormTarget::Password)
                                />
                            </label>
                            <label class="label">
                                <a href="#" class="label-text-alt link link-hover">
                                    Forgot password?
                                </a>
                            </label>
                        </div>
                        <div class="form-control mt-6">
                            <button
                                class="btn btn-primary"
                                on:click=handle_submit
                                disabled=move || login_loading.get()
                            >
                                <Show when=move || login_loading()>
                                    <span class="loading loading-spinner"></span>
                                </Show>
                                Login
                            </button>
                        </div>
                    </form>
                </div>
            </div>
        </div>
    }
}
