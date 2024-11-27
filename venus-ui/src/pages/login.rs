use ev::{Event, MouseEvent};
use gloo::net::http::Request;
use html::Form;
use leptos::*;
use serde::{Deserialize, Serialize};

use crate::{utils::error_to_string, GlobalUI, Notification};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BaseResponse<T> {
    pub code: i64,
    pub message: String,
    pub data: Option<T>,
}

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
    let address = format!("{}/api/user/login", login_form.server);
    let login_body = LoginBody {
        username: login_form.username,
        password: login_form.password,
    };
    let request = Request::post(&address)
        .header("Content-Type", "application/json")
        .body(serde_json::to_string(&login_body).map_err(error_to_string)?)
        .map_err(error_to_string)?
        .send()
        .await;
    match request {
        Ok(response) => response.json().await.map_err(error_to_string),
        Err(err) => Err(err.to_string()),
    }
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
    let (form, set_form) = create_signal(LoginForm {
        // server: "http://localhost:4000".into(),
        server: "http://192.168.1.57:4001".into(),
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

    let nts = use_context::<GlobalUI>()
        .expect("GlobalUI state is not set")
        .notifications;
    let form_ref = create_node_ref::<Form>();

    // 登录方法 点击登录按钮后触发
    let login_action = create_action(|login_form: &LoginForm| {
        let login_form = login_form.clone();
        async { login(login_form).await }
    });
    let login_loading = login_action.pending();
    let login_result = login_action.value();
    create_effect(move |_| {
        if login_result().is_none() {
            return;
        }
        let res = login_result().unwrap();
        match res {
            Ok(response) => match response.data {
                Some(data) => {
                    logging::log!("login response {:?}", data.access_token);
                    nts.update(|nts| {
                        nts.push(Notification {
                            key: nts.len() as u32,
                            kind: crate::NotificationKind::Success,
                            message: "Login success".into(),
                        })
                    });
                }
                None => {
                    nts.update(|nts| {
                        nts.push(Notification {
                            key: nts.len() as u32,
                            kind: crate::NotificationKind::Error,
                            message: response.message,
                        })
                    });
                }
            },
            Err(err) => {
                logging::error!("login error {:?}", err);
                nts.update(|nts| {
                    nts.push(Notification {
                        key: nts.len() as u32,
                        kind: crate::NotificationKind::Error,
                        message: "Login failed".into(),
                    })
                });
            }
        }
    });
    let handle_submit = move |e: MouseEvent| {
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
                            <button class="btn btn-primary" on:click=handle_submit>
                                {move || if login_loading() { "Loading..." } else { "Login" }}
                            // Login
                            </button>
                        </div>
                    </form>
                </div>
            </div>
        </div>
    }
}
