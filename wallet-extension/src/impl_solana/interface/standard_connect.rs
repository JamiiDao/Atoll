use solana_signer::Signer;
use wasm_bindgen::JsValue;
use zeroize::Zeroizing;

use crate::{
    AtollWalletError, AtollWalletResult, SolanaAccountKeypair, SolanaConstants,
    SolanaWalletAccount, TEST_MNEMONIC, TEST_PASSPHRASE, app_console_log,
};

pub fn standard_connect(data: &JsValue) -> AtollWalletResult<JsValue> {
    app_console_log(SolanaConstants::STANDARD_CONNECT, data);

    let wallet_account_keypair = SolanaAccountKeypair::new_from_mnemonic(
        Zeroizing::new(TEST_MNEMONIC.to_string()),
        Some(Zeroizing::new(TEST_PASSPHRASE.to_string())),
    )
    .or(Err(
        AtollWalletError::UnableToRecoverSolanaKeypairFromMnemonic,
    ))?;

    let public_key = wallet_account_keypair.expose().pubkey().to_bytes();

    let wallet_account = SolanaWalletAccount::new(public_key);

    Ok(wallet_account.to_js_value_object())
}
