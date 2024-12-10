use gloo::net::http::Request;
use leptos::prelude::on_cleanup;
use send_wrapper::SendWrapper;
use serde::{de::DeserializeOwned, Serialize};
use web_sys::wasm_bindgen::JsValue;

use crate::utils::error_to_string;

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

    let request = Request::post(address)
        .abort_signal(abort_signal.as_ref())
        .header("Content-Type", "application/json")
        .body(body)
        .map_err(error_to_string)?
        .send()
        .await;
    match request {
        Ok(response) => response.json().await.map_err(error_to_string),
        Err(err) => Err(err.to_string()),
    }
}
