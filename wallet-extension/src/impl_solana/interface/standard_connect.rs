use wasm_bindgen::JsValue;

use crate::{
    App, AtollWalletError, AtollWalletResult, KeypairOps, SolanaConstants, app_console_log,
};

impl App {
    pub async fn standard_connect(
        active_hash: blake3::Hash,
        keypair_ops: KeypairOps,
        data: &JsValue,
    ) -> AtollWalletResult<JsValue> {
        app_console_log(SolanaConstants::STANDARD_CONNECT, data);

        let uri = data.as_string().ok_or(AtollWalletError::JsCast(
            "JsValue for window URI requesting standard:connect is not a String.".to_string(),
        ))?;

        if let Some(active_keypair) = keypair_ops.write().await.get_mut(&active_hash) {
            let wallet_account = active_keypair.standard_connect(uri);

            Ok(wallet_account.to_js_value_object())
        } else {
            Err(AtollWalletError::UnauthorizedKeypairRequest)
        }
    }
}
