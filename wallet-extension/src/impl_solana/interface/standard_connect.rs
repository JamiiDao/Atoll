use wasm_bindgen::JsValue;

use crate::{App, AtollWalletError, AtollWalletResult, SolanaConstants, app_console_log};

impl App {
    pub fn standard_connect(&mut self, data: &JsValue) -> AtollWalletResult<JsValue> {
        app_console_log(SolanaConstants::STANDARD_CONNECT, data);

        let uri = data.as_string().ok_or(AtollWalletError::JsCast(
            "JsValue for window URI requesting standard:connect is not a String.".to_string(),
        ))?;

        let wallet_account = self.active_keypair()?.standard_connect(uri);

        Ok(wallet_account.to_js_value_object())
    }
}
