use ev::{Event, MouseEvent};
use gloo::net::http::Request;
use html::Form;
use leptos::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct BaseResponse<T> {
    pub code: i64,
    pub message: String,
    pub data: T,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Data {
    pub access_token: String,
    pub token_type: String,
}

type LoginResponse = BaseResponse<Data>;

async fn login(login_form: LoginForm) -> Result<LoginResponse, String> {
    let response = Request::post("http://192.168.1.57:4001/api/user/login")
        .header("Content-Type", "application/json")
        .body(
            serde_json::to_string(&login_form)
                .map_err(|_| "serialize to string failed".to_string())?,
        )
        .map_err(|_| "create body failed".to_string())?
        .send()
        .await
        .map_err(|_| "send request failed".to_string())?;

    if response.status() == 200 {
        Ok(response
            .json()
            .await
            .map_err(|_| "parse response failed".to_string())?)
    } else {
        Err(response.status_text())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct LoginForm {
    pub username: String,
    pub password: String,
}

#[component]
pub fn Login() -> impl IntoView {
    let (form, set_form) = create_signal(LoginForm {
        username: "".into(),
        password: "".into(),
    });

    enum FormTarget {
        Username,
        Password,
    }
    let handle_change = move |target: FormTarget| {
        use FormTarget::*;
        move |e: Event| {
            let value = event_target_value(&e);
            match target {
                Username => set_form.update(|f| f.username = value),
                Password => set_form.update(|f| f.password = value),
            }
        }
    };

    let login_action = create_action(|login_form: &LoginForm| {
        let login_form = login_form.clone();
        async {
            let response = login(login_form).await;
            if let Ok(response) = response {
                logging::log!("login response {:?}", response);
            };
        }
    });
    let form_ref = create_node_ref::<Form>();
    let handle_submit = move |e: MouseEvent| {
        e.prevent_default();
        let form_ref = form_ref.get().expect("form element is not rendered");
        let valided = form_ref.check_validity();
        if !valided {
            form_ref.report_validity();
            return;
        }
        logging::log!("username {} password {}", form().username, form().password);
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
                            <span class="label-text mb-2">Username</span>
                            <label class="input input-bordered flex items-center gap-2">
                                <img
                                    src="public/images/login/user.svg"
                                    alt="user icon"
                                    class="h-4 w-4 opacity-70"
                                />
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
                            <span class="label-text mb-2">Password</span>
                            <label class="input input-bordered flex items-center gap-2">
                                <img
                                    src="public/images/login/password.svg"
                                    alt="password icon"
                                    class="h-4 w-4 opacity-70"
                                />
                                <input
                                    type="password"
                                    class="grow"
                                    prop:value=move || form().password
                                    placeholder="Password"
                                    required
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
                                Login
                            </button>
                        </div>
                    </form>
                </div>
            </div>
        </div>
    }
}
