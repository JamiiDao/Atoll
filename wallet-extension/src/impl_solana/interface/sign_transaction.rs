use solana_transaction::Transaction;
use wasm_bindgen::{JsCast, JsValue};
use web_sys::js_sys::{self, Array, Uint8Array};

use crate::{
    App, AtollWalletError, AtollWalletResult, KeypairOps, Reflection, SolanaConstants,
    app_console_log,
};

impl App {
    pub async fn solana_sign_transaction(
        active_hash: blake3::Hash,
        keypair_ops: KeypairOps,
        data: JsValue,
    ) -> AtollWalletResult<JsValue> {
        app_console_log(SolanaConstants::SIGN_TRANSACTION, &data);
        let data = Reflection::new_object_from_js_value(data)?;

        let data = data.get_object(
            "requestData",
            AtollWalletError::JsCast("`requestData` not found in `message` object".to_string()),
        )?;

        let data = Reflection::new_object_from_js_value(data)?;
        let account_js_value = data.get_object(
            "account",
            AtollWalletError::JsCast(
                "`account` object was not found in the `requestData` for `solana:signTransaction`"
                    .to_string(),
            ),
        )?;
        let account_object = Reflection::new_object_from_js_value(account_js_value)?;
        let public_key: [u8;32] = account_object.get_object(
            "publicKey",
            AtollWalletError::JsCast(
                "`publicKey` was not found in the `requestData.accounts` for `solana:signTransaction`"
                    .to_string(),
            ),
        )?.dyn_into::<js_sys::Uint8Array>().or(
            Err(AtollWalletError::JsCast("`publicKey` in `requestData` for `solana:signTransaction` not a byte array.".to_string()))
        )?.to_vec().try_into().or(Err(AtollWalletError::Input("`publicKey` in `requestData` for `solana:signTransaction` not an byte array of 32 in length.".to_string())))?;

        let transaction_js_value = data.get_object(
            "transaction",
            AtollWalletError::JsCast(
                "`transaction` was not found in the `requestData` for `solana:signTransaction`"
                    .to_string(),
            ),
        )?;

        let transaction_bytes = transaction_js_value
            .dyn_into::<js_sys::Uint8Array>()
            .or(Err(AtollWalletError::JsCast(
                "`message` in `requestData` for `solana:signTransaction` not a byte array."
                    .to_string(),
            )))?
            .to_vec();

        let transaction = bincode::deserialize::<Transaction>(&transaction_bytes).or(Err(
            AtollWalletError::Input("Transaction for `solana:signTransaction` is invalid. Try constructing the transaction correctly!".to_string())
        ))?;

        if let Some(active_keypair) = keypair_ops.write().await.get_mut(&active_hash) {
            let signed_transaction = active_keypair.sign_transaction(&public_key, transaction);
            let signed_transaction_bytes = bincode::serialize(&signed_transaction).or(Err(
                AtollWalletError::Input("Unable to encode signed transaction".to_string()),
            ))?;

            let signed_transaction_output = Reflection::new_object();

            let signed_transaction_uint8array =
                Uint8Array::new_from_slice(&signed_transaction_bytes);
            signed_transaction_output
                .set_object_secure("signedTransaction", &signed_transaction_uint8array);

            let output_array = Array::new();
            output_array.push(&signed_transaction_output.take());

            Ok(output_array.into())
        } else {
            Err(AtollWalletError::UnauthorizedKeypairRequest)
        }
    }
}
