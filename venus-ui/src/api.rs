use core::fmt;

use gloo::net::http::Request;
use leptos::{logging, prelude::on_cleanup};
use send_wrapper::SendWrapper;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use web_sys::wasm_bindgen::JsValue;

use crate::{utils::error_to_string, USER};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BaseResponse<T> {
    pub code: i64,
    pub message: String,
    pub data: Option<T>,
}

/// Send a POST request to the server
///
/// ## Arguments
/// * `address` - The address of the server
/// * `body` - The body of the request
///
/// ## Returns
/// The response from the server
pub async fn post<T>(address: &str, body: impl Into<JsValue>) -> Result<T, String>
where
    T: Serialize + DeserializeOwned,
{
    let abort_controller = SendWrapper::new(web_sys::AbortController::new().ok());
    let abort_signal = abort_controller.as_ref().map(|a| a.signal());

    // abort in-flight requests if, e.g., we've navigated away from this page
    on_cleanup(move || {
        if let Some(abort_controller) = abort_controller.take() {
            abort_controller.abort()
        }
    });

    let user = USER.read();
    let (token_type, token) = match user {
        Ok(user) => (user.token_type.clone(), user.token.clone()),
        Err(err) => {
            logging::error!("cannot get user {err}");
            ("".to_string(), "".to_string())
        }
    };

    let request = Request::post(address)
        .abort_signal(abort_signal.as_ref())
        .header("Content-Type", "application/json")
        .header(
            "Authorization",
            format!("{} {}", token_type, token).as_str(),
        )
        .body(body)
        .map_err(error_to_string)?
        .send()
        .await;
    match request {
        Ok(response) => response.json().await.map_err(error_to_string),
        Err(err) => Err(err.to_string()),
    }
}

pub enum RequestApi {
    Login,
    AddSubscription,
}
impl fmt::Display for RequestApi {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Login => write!(f, "/api/user/login"),
            Self::AddSubscription => write!(f, "/api/subscription/add"),
        }
    }
}
