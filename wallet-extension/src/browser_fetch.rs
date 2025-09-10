use wasm_bindgen::JsCast;
use wasm_bindgen_futures::JsFuture;
use web_sys::{Headers, RequestInit};

use crate::{AtollWalletError, AtollWalletResult};

pub struct BrowserFetch {
    headers: Headers,
    options: RequestInit,
}

impl BrowserFetch {
    pub fn new() -> AtollWalletResult<Self> {
        let headers = Headers::new().or(Err(AtollWalletError::Input(
            "Unable to instantiate `Headers::new()` for browser fetch API".to_string(),
        )))?;

        let options = RequestInit::new();
        options.set_method("POST");

        headers
            .append("content-type", "application/json")
            .or(Err(AtollWalletError::Input(
                "Unable to append `Content-Type: application/json` header for browser fetch API"
                    .to_string(),
            )))?;
        headers
            .append("Accept", "application/json")
            .or(Err(AtollWalletError::Input(
                "Unable to append `Accept: application/json` header for browser fetch API"
                    .to_string(),
            )))?;

        Ok(Self { headers, options })
    }

    pub fn set_body(&mut self, json_body: &str) -> &mut Self {
        self.options.set_body(&json_body.into());

        self
    }

    pub async fn send(self, url: &str) -> AtollWalletResult<web_sys::Response> {
        self.options.set_headers(&self.headers);

        let request = web_sys::Request::new_with_str_and_init(url, &self.options).or(Err(
            AtollWalletError::Input("Unable to construct a request to send to an RPC".to_string()),
        ))?;

        let window = web_sys::window().ok_or(AtollWalletError::JsCast(
            "Window not found. A request will not be sent to the RPC".to_string(),
        ))?;

        let fetch_promise = window.fetch_with_request(&request);

        // Await the fetch promise to get a `Response` object
        let resp_value = JsFuture::from(fetch_promise)
            .await
            .or(Err(AtollWalletError::JsCast(
                "Unable to send request to the browser".to_string(),
            )))?;

        resp_value
            .dyn_into::<web_sys::Response>()
            .or(Err(AtollWalletError::JsCast(
                "Unable to cast the browser fetch response to a `web_sys::Response` type"
                    .to_string(),
            )))
    }
}
