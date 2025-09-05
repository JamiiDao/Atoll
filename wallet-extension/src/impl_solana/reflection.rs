//  Reflect::set(
//             &wallet_account_object,
//             &"address".into(),
//             &self.address.as_ref().into(),
//         )
//         .or(Err(AtollWalletError::UnableToSetWalletAccountAddress))?;

use wasm_bindgen::{JsCast, JsValue};
use web_sys::{
    console,
    js_sys::{self, Reflect},
};

use crate::{AtollWalletError, AtollWalletResult};

#[derive(Debug)]
pub struct Reflection(JsValue);

impl Reflection {
    pub fn new(js_value: JsValue) -> Self {
        Self(js_value)
    }

    /// Initialize new [Reflection] from a [js_sys::Object]
    pub fn new_object() -> Self {
        Self(js_sys::Object::new().into())
    }

    /// Initialize [Reflection] from a [JsValue] of type [js_sys::Object]
    pub fn new_object_from_js_value(js_value: JsValue) -> AtollWalletResult<Self> {
        js_value.as_ref().dyn_ref::<js_sys::Object>().ok_or(
            AtollWalletError::JsValueIsNotAnObject(Self::js_typeof(&js_value)?),
        )?;

        Ok(Self(js_value))
    }

    pub fn new_uint8_array(array: &[u8]) -> JsValue {
        let new_array = js_sys::Uint8Array::new_with_length(array.len() as u32);
        new_array.copy_from(array);

        new_array.into()
    }

    pub fn new_str_array(array: &[&str]) -> JsValue {
        let new_array = js_sys::Array::new_with_length(array.len() as u32);

        array.iter().for_each(|value| {
            let inner_value: JsValue = (*value).into();
            new_array.push(&inner_value);
        });

        new_array.into()
    }

    pub fn set_object(
        &self,
        key: &str,
        js_value: &JsValue,
        error: AtollWalletError,
    ) -> AtollWalletResult<&Self> {
        Reflect::set(&self.0, &key.into(), js_value).or(Err(error))?;

        Ok(self)
    }

    /// This panics if not handled properly
    pub fn set_object_secure(&self, key: &str, js_value: &JsValue) -> &Self {
        if let Err(error) = Reflect::set(&self.0, &key.into(), js_value) {
            console::log_7(
                &"AtollWallet > Unable to set object for key `".into(),
                &key.into(),
                &"` for value > `".into(),
                js_value,
                &"`. Js Error: `".into(),
                &error,
                &"`. Try using `set_object` instead which returns an error you can handle.".into(),
            );
        }

        self
    }

    pub fn get_object(&self, key: &str, error: AtollWalletError) -> AtollWalletResult<JsValue> {
        Reflect::get(&self.0, &key.into()).or(Err(error))
    }

    pub fn take(self) -> JsValue {
        self.0
    }

    pub fn cloned(&self) -> JsValue {
        self.0.clone()
    }

    pub fn peek(&self) -> &JsValue {
        self.0.as_ref()
    }

    pub fn js_typeof(js_value: &JsValue) -> AtollWalletResult<String> {
        js_value
            .js_typeof()
            .as_string()
            .ok_or(AtollWalletError::UnableToCheckTypeOfJsValue(
                js_value.clone(),
            ))
    }
}
