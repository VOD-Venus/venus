use gloo::net::http::Request;
use leptos::web_sys::MouseEvent;
use leptos::{ev::Event, logging, prelude::*};
use serde::{Deserialize, Serialize};

use crate::{utils::error_to_string, GlobalUI, Notification, NotificationKind};

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
    use leptos::prelude::on_cleanup;
    use send_wrapper::SendWrapper;

    let abort_controller = SendWrapper::new(web_sys::AbortController::new().ok());
    let abort_signal = abort_controller.as_ref().map(|a| a.signal());

    // abort in-flight requests if, e.g., we've navigated away from this page
    on_cleanup(move || {
        if let Some(abort_controller) = abort_controller.take() {
            abort_controller.abort()
        }
    });

    let address = format!("{}/api/user/login", login_form.server);
    let login_body = LoginBody {
        username: login_form.username,
        password: login_form.password,
    };
    let request = Request::post(&address)
        .abort_signal(abort_signal.as_ref())
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

    let nts = use_context::<GlobalUI>()
        .expect("GlobalUI state is not set")
        .notifications;
    let form_ref: NodeRef<leptos::html::Form> = NodeRef::new();

    // 登录方法 点击登录按钮后触发
    let login_action: Action<
        LoginForm,
        std::result::Result<BaseResponse<Data>, String>,
        SyncStorage,
    > = Action::new_unsync(|login_form: &LoginForm| login(login_form.clone()));
    let login_loading = login_action.pending();
    let login_result = login_action.value();
    Effect::new(move |_| {
        if let Some(result) = login_result.read().as_ref() {
            match result {
                Ok(response) => {
                    if let Some(data) = &response.data {
                        logging::log!("login response {:?}", data.access_token);
                        nts.update(|nts| {
                            nts.push(Notification::new(
                                NotificationKind::Success,
                                "Login success".into(),
                            ));
                        });
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
        }
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
