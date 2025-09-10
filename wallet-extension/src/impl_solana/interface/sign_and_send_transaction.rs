use std::str::FromStr;

use serde::Deserialize;
use solana_transaction::Transaction;
use wallet_standard_base::Cluster;
use wasm_bindgen::{JsCast, JsValue};
use wasm_bindgen_futures::JsFuture;
use web_sys::js_sys::{self};

use crate::{
    App, AtollWalletError, AtollWalletResult, BrowserFetch, KeypairOps, Reflection, SendOptions,
    SolanaCluster, SolanaConstants, app_console_log,
};

impl App {
    pub async fn solana_sign_and_transaction(
        active_hash: blake3::Hash,
        keypair_ops: KeypairOps,
        data: JsValue,
    ) -> AtollWalletResult<JsValue> {
        app_console_log(SolanaConstants::SIGN_AND_SEND_TRANSACTION, &data);
        let data = Reflection::new_object_from_js_value(data)?;

        let data = data.get_object(
            "requestData",
            AtollWalletError::JsCast("`requestData` not found in `message` object".to_string()),
        )?;

        let data = Reflection::new_object_from_js_value(data)?;
        let account_js_value = data.get_object(
            "account",
            AtollWalletError::JsCast(
                "`account` object was not found in the `requestData` for `solana:signAndSendTransaction`"
                    .to_string(),
            ),
        )?;
        let account_object = Reflection::new_object_from_js_value(account_js_value)?;
        let public_key: [u8;32] = account_object.get_object(
            "publicKey",
            AtollWalletError::JsCast(
                "`publicKey` was not found in the `requestData.accounts` for `solana:signAndSendTransaction`"
                    .to_string(),
            ),
        )?.dyn_into::<js_sys::Uint8Array>().or(
            Err(AtollWalletError::JsCast("`publicKey` in `requestData` for `solana:signAndSendTransaction` not a byte array.".to_string()))
        )?.to_vec().try_into().or(Err(AtollWalletError::Input("`publicKey` in `requestData` for `solana:signAndSendTransaction` not an byte array of 32 in length.".to_string())))?;

        let transaction_js_value = data.get_object(
            "transaction",
            AtollWalletError::JsCast(
                "`transaction` was not found in the `requestData` for `solana:signAndSendTransaction`"
                    .to_string(),
            ),
        )?;

        let transaction_bytes = transaction_js_value
            .dyn_into::<js_sys::Uint8Array>()
            .or(Err(AtollWalletError::JsCast(
                "`message` in `requestData` for `solana:signAndSendTransaction` not a byte array."
                    .to_string(),
            )))?
            .to_vec();

        let mut transaction = bincode::deserialize::<Transaction>(&transaction_bytes).or(Err(
            AtollWalletError::Input("Transaction for `solana:signAndSendTransaction` is invalid. Try constructing the transaction correctly!".to_string())
        ))?;

        let cluster: SolanaCluster = data.get_object(
            "chain",
            AtollWalletError::JsCast(
                "`chain` was not found in the `requestData` for `solana:signAndSendTransaction`"
                    .to_string(),
            ),
        )?.as_string().ok_or(AtollWalletError::JsCast(
                "`chain` in `requestData` for `solana:signAndSendTransaction` is not a String"
                    .to_string(),
            ))?.as_str().into();

        let mut options = SendOptions::default();

        if let Ok(options_js_value) = data.get_object(
            "options",
            AtollWalletError::JsCast(
                "`options` object was not found in the `requestData` for `solana:signTransaction`"
                    .to_string(),
            ),
        ) {
            options.parse(options_js_value);
        }

        if let Some(active_keypair) = keypair_ops.write().await.get_mut(&active_hash) {
            let blockhash = GetBlockHash::fetch(cluster).await?;

            let blockhash = solana_hash::Hash::from_str(blockhash.result.value.blockhash.as_str())
                .or(Err(AtollWalletError::Input(
                    "invalid blockhash String".to_string(),
                )))?;

            transaction.message.recent_blockhash = blockhash;

            let json_string = active_keypair
                .sign_and_send_transaction(public_key, transaction, options, blockhash, cluster)
                .await?;

            if let Ok(success) = serde_json::from_str::<RpcResponse>(&json_string) {
                let signature = bs58::decode(&success.result.as_bytes()).into_vec().or(Err(
                    AtollWalletError::JsCast("Invalid Base58 from response signature".to_string()),
                ))?;
                let signature = js_sys::Uint8Array::new_from_slice(&signature);
                let signature_object = Reflection::new_object();
                signature_object.set_object_secure("signature", &signature);

                let signature_array = js_sys::Array::new();
                signature_array.push(&signature_object.take());

                Ok(signature_array.into())
            } else if let Ok(failure) = serde_json::from_str::<RpcResponseError>(&json_string) {
                Err(AtollWalletError::Input(format!(
                    "Encountered error when deserializing JSON response in `solana:sendAndSignTransaction`. Error: {}",
                    failure.error.message
                )))
            } else {
                Ok(json_string.into())
            }
        } else {
            Err(AtollWalletError::UnauthorizedKeypairRequest)
        }
    }
}

#[derive(Debug, Deserialize, PartialEq, Eq, Clone)]
struct RpcResponse {
    jsonrpc: String,
    result: String,
    id: u8,
}

#[derive(Debug, Deserialize, PartialEq, Eq, Clone)]
struct RpcResponseError {
    jsonrpc: String,
    error: ErrorInfo,
    id: u8,
}

#[derive(Debug, Deserialize, PartialEq, Eq, Clone)]
struct ErrorInfo {
    code: u16,
    message: String,
}

#[derive(Debug, Deserialize, PartialEq, Eq, Clone)]
struct GetBlockHash {
    jsonrpc: String,
    result: GetBlockHashResult,
    id: u8,
}

impl GetBlockHash {
    async fn fetch(cluster: SolanaCluster) -> AtollWalletResult<Self> {
        let mut fetch = BrowserFetch::new()?;
        let body = jzon::object! {
          "jsonrpc": "2.0",
          "id": 1,
          "method": "getLatestBlockhash",
          "params": [
            {
              "commitment": "finalized"
            }
          ]
        }
        .to_string();

        fetch.set_body(body.as_str());
        let response_result = fetch.send(cluster.endpoint()).await?;

        let response_text = response_result.text().or(Err(AtollWalletError::JsCast("Unable to get the JSON body after the response was send in `solana:sendAndSignTransaction` getLatestBlockhash".to_string())))?;
        let response_text = JsFuture::from(response_text).await.map_err(|error| {
            AtollWalletError::JsCast(format!("Unable to get response body from the response of getLatestBlockhash in `solana:sendAndSignTransaction`: Error: {error:?}"))
        })?;
        let response_str = response_text.as_string().ok_or(AtollWalletError::JsCast(
            "Response from getLatestlockhash is not run".to_string(),
        ))?;

        serde_json::from_str::<Self>(&response_str).or(Err(AtollWalletError::JsCast(
            "Unable to resolve the json body from the RPC response ingetLatestBlockhash"
                .to_string(),
        )))
    }
}

#[derive(Debug, Deserialize, PartialEq, Eq, Clone)]
struct GetBlockHashResult {
    value: BlockHashValue,
}

#[derive(Debug, Deserialize, PartialEq, Eq, Clone)]
struct BlockHashValue {
    blockhash: String,
}
