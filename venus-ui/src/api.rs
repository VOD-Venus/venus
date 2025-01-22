use core::fmt;

use gloo::net::http::{Method, RequestBuilder};
use leptos::{logging, prelude::on_cleanup};
use send_wrapper::SendWrapper;
use serde::{Deserialize, Serialize};

use crate::USER;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BaseResponse<T> {
    pub code: i64,
    pub message: String,
    pub data: Option<T>,
}

/// Create a new RequestBuilder
///
/// # Parameters
/// * `address` - The address of the server
/// * `method` - The request method
pub fn axios(address: &str, method: Method) -> RequestBuilder {
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

    let mut client = RequestBuilder::new(address)
        .method(method.clone())
        .abort_signal(abort_signal.as_ref());
    if !token.is_empty() {
        client = client.header(
            "Authorization",
            format!("{} {}", token_type, token).as_str(),
        );
    }
    client
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
