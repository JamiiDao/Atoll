use std::{borrow::Cow, collections::HashMap};

use base64ct::{Base64, Encoding};
use bip39::{Language, Mnemonic, MnemonicType};
use solana_keypair::Keypair;
use solana_pubkey::Pubkey;
use solana_seed_derivable::SeedDerivable;
use solana_signer::Signer;
use solana_transaction::Transaction;
use wallet_standard_base::{Cluster, Commitment};
use wasm_bindgen::JsValue;
use wasm_bindgen_futures::JsFuture;
use zeroize::Zeroizing;

use crate::{
    AtollWalletError, AtollWalletResult, BrowserFetch, Reflection, SolanaCluster, SolanaCommitment,
    SolanaWalletAccount,
};

const TEST_PASSPHRASE: &str = "quick brown fox";

const TEST_MNEMONIC: &str =
    "wrap kingdom punch clog kiss useless celery exist bulk catch share creek";

pub struct SolanaAccountKeypair {
    keypair: Keypair,
    active_dapps: HashMap<blake3::Hash, ActiveDapp>,
}

impl<'wa> SolanaAccountKeypair {
    pub(crate) fn new_test() -> AtollWalletResult<Self> {
        Self::new_from_mnemonic(
            Zeroizing::new(TEST_MNEMONIC.to_string()),
            Some(Zeroizing::new(TEST_PASSPHRASE.to_string())),
        )
    }

    pub(crate) fn new_from_mnemonic(
        mnemonic: Zeroizing<String>,
        passphrase: Option<Zeroizing<String>>,
    ) -> AtollWalletResult<Self> {
        let mnemonic = Mnemonic::from_phrase(&mnemonic, Language::English)?;

        let phrase: &str = mnemonic.phrase();

        let keypair =
            Keypair::from_seed_phrase_and_passphrase(phrase, &passphrase.unwrap_or_default())
                .map_err(|error| {
                    AtollWalletError::UnableToConvertMnemonicToKeypair(error.to_string())
                })?;

        Ok(Self {
            keypair,
            active_dapps: HashMap::default(),
        })
    }

    pub(crate) fn _new(
        passphrase: Option<Zeroizing<String>>,
    ) -> AtollWalletResult<(Self, Zeroizing<String>)> {
        let mnemonic = Mnemonic::new(MnemonicType::Words12, Language::English);

        let phrase = mnemonic.phrase().to_owned();

        let keypair =
            Keypair::from_seed_phrase_and_passphrase(&phrase, &passphrase.unwrap_or_default())
                .map_err(|error| {
                    AtollWalletError::UnableToConvertMnemonicToKeypair(error.to_string())
                })?;

        Ok((
            Self {
                keypair,
                active_dapps: HashMap::default(),
            },
            Zeroizing::new(phrase),
        ))
    }

    pub(crate) fn pubkey(&self) -> Pubkey {
        self.keypair.pubkey()
    }

    pub fn standard_connect(&'wa mut self, uri: String) -> SolanaWalletAccount<'wa> {
        let (active_dapp, hash) = ActiveDapp::new(uri);
        self.active_dapps.insert(hash, active_dapp);

        let public_key = self.pubkey().to_bytes();

        SolanaWalletAccount::new(public_key)
    }

    pub fn sign_in(&'wa mut self, formatted_input: &str) -> (SolanaWalletAccount<'wa>, [u8; 64]) {
        let signature = self.keypair.sign_message(formatted_input.as_bytes());

        // TODO add to active dapps and add dapp icon too

        (self.get_wallet_account(), *signature.as_array())
    }

    // TODO type checks to see if a dapp is currently authorized to perform an operation
    pub fn sign_message(&self, _public_key: &[u8; 32], message: &[u8]) -> [u8; 64] {
        let signature = self.keypair.sign_message(message);

        *signature.as_array()
    }

    // TODO type checks to see if a dapp is currently authorized to perform an operation
    pub fn sign_transaction(
        &self,
        _public_key: &[u8; 32],
        mut transaction: Transaction,
    ) -> Transaction {
        let recent_blockhash = transaction.message.hash();
        transaction.sign(&[&self.keypair], recent_blockhash);

        transaction
    }

    // TODO type checks to see if a dapp is currently authorized to perform an operation
    // TODO Use getSignatureStatuses to ensure a transaction is processed and confirmed.
    pub async fn sign_and_send_transaction(
        &self,
        _public_key: [u8; 32],
        mut transaction: Transaction,
        send_options: crate::SendOptions,
        recent_blockhash: solana_hash::Hash,
        cluster: SolanaCluster,
    ) -> AtollWalletResult<String> {
        transaction.sign(&[&self.keypair], recent_blockhash);

        let mut fetch = BrowserFetch::new()?;

        let signed_transaction_bytes = bincode::serialize(&transaction).or(Err(
            AtollWalletError::Input("Unable to convert the signed transaction into bytes for `solana:signAndSendTransaction`".to_string())
        ))?;

        let encoded_signed_transaction = Base64::encode_string(&signed_transaction_bytes);
        let json_body = jzon::object! {
          "jsonrpc": "2.0",
          "id": 1,
          "method": "sendTransaction",
          "params": [
            encoded_signed_transaction,
            send_options.to_json()
          ]
        }
        .to_string();
        fetch.set_body(&json_body);

        let success = fetch
            .send(cluster.endpoint())
            .await?
            .text()
            .map_err(|error| {
                AtollWalletError::JsCast(format!(
                    "Unable to get the text from response body: Error: {error:?}"
                ))
            })?;
        let success = JsFuture::from(success).await.or(
            Err(AtollWalletError::JsCast("Unable to get the JSON body after the response was send in `solana:sendAndSignTransaction`".to_string()))
        )?;

        let success = success.as_string().ok_or(AtollWalletError::JsCast("Unable to get the JSON body after the response was send in `solana:sendAndSignTransaction`".to_string()))?;

        Ok(success)
    }

    pub fn get_wallet_account(&'wa self) -> SolanaWalletAccount<'wa> {
        let public_key = self.pubkey().to_bytes();

        SolanaWalletAccount::new(public_key)
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct ActiveDapp {
    uri: Cow<'static, str>,
    sign_in: Option<SignInValues>,
}

impl ActiveDapp {
    pub fn new(uri: String) -> (Self, blake3::Hash) {
        let hash = blake3::hash(uri.as_bytes());
        let new_self = Self {
            uri: Cow::Owned(uri),
            sign_in: Option::default(),
        };

        (new_self, hash)
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct SignInValues;

#[derive(Debug, Default)]
pub struct SendOptions {
    pub preflight_commitment: SolanaCommitment,
    pub skip_preflight: bool,
    pub max_retries: usize,
}

impl SendOptions {
    pub fn new() -> Self {
        Self::default()
    }

    // TODO parse minContextSlot
    pub fn parse(&mut self, options_js_value: JsValue) -> &mut Self {
        if let Ok(options_object) = Reflection::new_object_from_js_value(options_js_value)
            && let Ok(preflight_commitment) = options_object.get_object(
            "preflightCommitment",
            AtollWalletError::JsCast(
                "`preflightCommitment` object was not found in the `requestData.options` for `solana:signTransaction`"
                    .to_string(),
            ),
        ){
            if let Some(value) = preflight_commitment.as_string() { self.preflight_commitment = value.as_str().into(); }

        if let Ok(skip_preflight) = options_object.get_object(
            "skipPreflight",
            AtollWalletError::JsCast(
                "`skipPreflight` object was not found in the `requestData.options` for `solana:signTransaction`"
                    .to_string(),
            ),
        )
            && let Some(value) = skip_preflight.as_bool() { self.skip_preflight = value; }

         if let Ok(max_retries) = options_object.get_object(
            "maxRetries",
            AtollWalletError::JsCast(
                "`maxRetries` object was not found in the `requestData.options` for `solana:signTransaction`"
                    .to_string(),
            ),
        )
            && let Some(value) = max_retries.as_f64() { self.max_retries = value as usize; }
        }

        self
    }

    pub fn to_json(&self) -> jzon::JsonValue {
        jzon::object! {
            preflightCommitment: self.preflight_commitment.as_str(),
            skip_preflight: self.skip_preflight,
            max_retries: self.max_retries,
            encoding: "base64"
        }
    }
}
