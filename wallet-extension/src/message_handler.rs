use std::panic;

use wasm_bindgen::prelude::*;
use web_sys::{
    console,
    js_sys::{self, Function, Reflect},
};

use crate::{
    AtollWalletError, AtollWalletResult, Reflection, SolanaConstants, WasmOutcome, standard_connect,
};

#[wasm_bindgen]
pub fn app(extension: JsValue) {
    panic::set_hook(Box::new(console_error_panic_hook::hook));

    let runtime = Reflect::get(&extension, &JsValue::from_str("runtime")).unwrap_or_else(|_| {
        panic!(
            "{}",
            AtollWalletError::ExtensionRuntimeIsMissing.to_string()
        )
    });

    let on_message = Reflect::get(&runtime, &JsValue::from_str("onMessage")).unwrap_or_else(|_| {
        panic!(
            "{}",
            AtollWalletError::ExtensionRuntimeMessageIsMissing.to_string()
        )
    });

    let add_listener = Reflect::get(&on_message, &JsValue::from_str("addListener"))
        .unwrap_or_else(|_| {
            panic!(
                "{}",
                AtollWalletError::ExtensionRuntimeMessageAddListenerIsMissing.to_string()
            )
        })
        .dyn_into::<js_sys::Function>()
        .unwrap_or_else(|_| {
            panic!("{}", AtollWalletError::JsCast(
                "Unable to convert `extension.runtime.onMessage.addListener` to js_sys::Function"
                    .to_string(),
            )
            .to_string())
        });

    let send_response_callback = Closure::wrap(Box::new(
        move |message: JsValue, _sender: JsValue, send_response: JsValue| {
            let processed_message = match_message(message);
            let reply = WasmOutcome::new(processed_message);

            let send_response_fn = send_response
                .dyn_into::<Function>()
                .expect("Unable to convert a `sendResponse` for `extension.runtime.onMessage.addListener` to a js_sys::Function");

            send_response_fn
                .call1(&JsValue::NULL, &reply.to_object())
                .expect("Unable to call `sendResponse`");
        },
    ) as Box<dyn FnMut(JsValue, JsValue, JsValue)>);

    let send_response_callback_fn = send_response_callback
        .as_ref()
        .dyn_ref::<Function>()
        .expect("Unable to convert a `send_response_callback` to a js_sys::Function");

    add_listener
        .call1(&on_message, send_response_callback_fn)
        .expect("Calling `send_response_callback` error");

    send_response_callback.forget();
}

fn match_message(message: JsValue) -> AtollWalletResult<JsValue> {
    let message_object = Reflection::new_object_from_js_value(message)?;

    let resource_js_value = message_object.get_object(
        "resource",
        AtollWalletError::ResourceNotFoundInMessageObject,
    )?;

    let data = message_object.get_object("data", AtollWalletError::DataNotFoundInMessageObject)?;

    let resource: ExtensionMessage = resource_js_value.as_ref().try_into()?;

    match resource {
        ExtensionMessage::StandardConnect => standard_connect(&data),
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub enum ExtensionMessage {
    StandardConnect,
}

impl TryFrom<&JsValue> for ExtensionMessage {
    type Error = AtollWalletError;

    fn try_from(js_value: &JsValue) -> Result<Self, Self::Error> {
        let parsed_js_value = js_value.as_string().ok_or(AtollWalletError::JsCast(
            "The `resource` JsValue from the `extension.runtime.onMessage.addListener.message` is not a String type"
                .to_string(),
        ))?;
        let matched = match parsed_js_value.as_str() {
            SolanaConstants::STANDARD_CONNECT => Self::StandardConnect,
            _ => {
                return Err(AtollWalletError::UnsupportedExtensionMessage(
                    parsed_js_value,
                ));
            }
        };

        Ok(matched)
    }
}

pub fn app_console_log(event_name: &str, message: &JsValue) {
    #[cfg(debug_assertions)]
    console::log_3(&event_name.into(), &" ->> ".into(), message);
}

pub fn app_error_log(error: &AtollWalletError) {
    console::log_2(&"Extension Error ->> ".into(), &error.to_string().into());
}
