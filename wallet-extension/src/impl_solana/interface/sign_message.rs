use wasm_bindgen::{JsCast, JsValue};
use web_sys::js_sys::{self, Array, Uint8Array};

use crate::{
    App, AtollWalletError, AtollWalletResult, Reflection, SolanaConstants, app_console_log,
};

impl App {
    pub fn solana_sign_message(&mut self, data: JsValue) -> AtollWalletResult<JsValue> {
        app_console_log(SolanaConstants::SIGN_MESSAGE, &data);
        let data = Reflection::new_object_from_js_value(data)?;

        let data = data.get_object(
            "requestData",
            AtollWalletError::JsCast("`requestData` not found in `message` object".to_string()),
        )?;

        let data = Reflection::new_object_from_js_value(data)?;
        let account_js_value = data.get_object(
            "account",
            AtollWalletError::JsCast(
                "`account` object was not found in the `requestData` for `solana:signMessage`"
                    .to_string(),
            ),
        )?;
        let account_object = Reflection::new_object_from_js_value(account_js_value)?;
        let public_key: [u8;32] = account_object.get_object(
            "publicKey",
            AtollWalletError::JsCast(
                "`publicKey` was not found in the `requestData.accounts` for `solana:signMessage`"
                    .to_string(),
            ),
        )?.dyn_into::<js_sys::Uint8Array>().or(
            Err(AtollWalletError::JsCast("`publicKey` in `requestData` for `solana:signMessage` not a byte array.".to_string()))
        )?.to_vec().try_into().or(Err(AtollWalletError::Input("`publicKey` in `requestData` for `solana:signMessage` not an byte array of 32 in length.".to_string())))?;
        let message_js_value = data.get_object(
            "message",
            AtollWalletError::JsCast(
                "`message` was not found in the `requestData` for `solana:signMessage`".to_string(),
            ),
        )?;

        let message_bytes = message_js_value
            .dyn_into::<js_sys::Uint8Array>()
            .or(Err(AtollWalletError::JsCast(
                "`message` in `requestData` for `solana:signMessage` not a byte array.".to_string(),
            )))?
            .to_vec();

        let signature = self
            .active_keypair()?
            .sign_message(&public_key, &message_bytes);

        let sign_in_output = Reflection::new_object();

        let signed_message = Uint8Array::new_from_slice(message_bytes.as_slice());
        sign_in_output.set_object_secure("signedMessage", &signed_message);

        let signature = Uint8Array::new_from_slice(&signature);
        sign_in_output.set_object_secure("signature", &signature);

        sign_in_output.set_object_secure("signatureType", &"ed25519".into());

        let output_array = Array::new();
        output_array.push(&sign_in_output.take());

        Ok(output_array.into())
    }
}
