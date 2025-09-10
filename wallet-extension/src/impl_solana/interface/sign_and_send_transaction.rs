// use serde::Deserialize;
// use solana_transaction::Transaction;
// use wasm_bindgen::{JsCast, JsValue};
// use web_sys::js_sys::{self};

// use crate::{
//     App, AtollWalletError, Reflection, SendOptions, SolanaCluster, SolanaConstants, app_console_log,
// };

// impl App {
//     pub async fn solana_sign_and_transaction(&mut self, data: JsValue) -> Result<JsValue, JsValue> {
//         app_console_log(SolanaConstants::SIGN_AND_SEND_TRANSACTION, &data);
//         let data = Reflection::new_object_from_js_value(data)?;

//         let data = data.get_object(
//             "requestData",
//             AtollWalletError::JsCast("`requestData` not found in `message` object".to_string()),
//         )?;

//         let data = Reflection::new_object_from_js_value(data)?;
//         let account_js_value = data.get_object(
//             "account",
//             AtollWalletError::JsCast(
//                 "`account` object was not found in the `requestData` for `solana:signAndSendTransaction`"
//                     .to_string(),
//             ),
//         )?;
//         let account_object = Reflection::new_object_from_js_value(account_js_value)?;
//         let public_key: [u8;32] = account_object.get_object(
//             "publicKey",
//             AtollWalletError::JsCast(
//                 "`publicKey` was not found in the `requestData.accounts` for `solana:signAndSendTransaction`"
//                     .to_string(),
//             ),
//         )?.dyn_into::<js_sys::Uint8Array>().or(
//             Err(AtollWalletError::JsCast("`publicKey` in `requestData` for `solana:signAndSendTransaction` not a byte array.".to_string()))
//         )?.to_vec().try_into().or(Err(AtollWalletError::Input("`publicKey` in `requestData` for `solana:signAndSendTransaction` not an byte array of 32 in length.".to_string())))?;

//         let transaction_js_value = data.get_object(
//             "transaction",
//             AtollWalletError::JsCast(
//                 "`transaction` was not found in the `requestData` for `solana:signAndSendTransaction`"
//                     .to_string(),
//             ),
//         )?;

//         let transaction_bytes = transaction_js_value
//             .dyn_into::<js_sys::Uint8Array>()
//             .or(Err(AtollWalletError::JsCast(
//                 "`message` in `requestData` for `solana:signAndSendTransaction` not a byte array."
//                     .to_string(),
//             )))?
//             .to_vec();

//         let transaction = bincode::deserialize::<Transaction>(&transaction_bytes).or(Err(
//             AtollWalletError::Input("Transaction for `solana:signAndSendTransaction` is invalid. Try constructing the transaction correctly!".to_string())
//         ))?;

//         let cluster: SolanaCluster = data.get_object(
//             "chain",
//             AtollWalletError::JsCast(
//                 "`chain` was not found in the `requestData` for `solana:signAndSendTransaction`"
//                     .to_string(),
//             ),
//         )?.as_string().ok_or(AtollWalletError::JsCast(
//                 "`chain` in `requestData` for `solana:signAndSendTransaction` is not a String"
//                     .to_string(),
//             ))?.as_str().into();

//         let mut options = SendOptions::default();

//         if let Ok(options_js_value) = data.get_object(
//             "options",
//             AtollWalletError::JsCast(
//                 "`options` object was not found in the `requestData` for `solana:signTransaction`"
//                     .to_string(),
//             ),
//         ) {
//             options.parse(options_js_value);
//         }

//         self.active_keypair()
//             .await?
//             .sign_and_send_transaction(public_key, transaction, options, cluster)
//             .await
//             .map_err(|error| {
//                 let error: JsValue = error.to_string().into();

//                 error
//             })
//     }
// }

// #[derive(Debug, Deserialize, PartialEq, Eq, Clone)]
// struct RpcResponse {
//     jsonrpc: String,
//     result: String,
//     id: u8,
// }

// #[derive(Debug, Deserialize, PartialEq, Eq, Clone)]
// struct RpcResponseError {
//     jsonrpc: String,
//     error: ErrorInfo,
//     id: u8,
// }

// #[derive(Debug, Deserialize, PartialEq, Eq, Clone)]
// struct ErrorInfo {
//     code: u16,
//     message: String,
// }
