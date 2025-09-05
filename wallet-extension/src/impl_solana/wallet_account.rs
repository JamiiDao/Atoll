use core::fmt;
use std::borrow::Cow;

use bip39::{Language, Mnemonic, MnemonicType};
use solana_keypair::Keypair;
use solana_seed_derivable::SeedDerivable;
use wallet_standard_base::{
    Byte32Array, Cluster, ClusterEnabled, StandardFeatures, WalletAccount, WalletStandardIcon,
};
use wasm_bindgen::JsValue;
use zeroize::Zeroizing;

use crate::{AtollWalletError, AtollWalletResult, Reflection, SolanaCluster};

pub struct SolanaAccountKeypair(Keypair);

impl SolanaAccountKeypair {
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

        Ok(Self(keypair))
    }

    pub(crate) fn new(
        passphrase: Option<Zeroizing<String>>,
    ) -> AtollWalletResult<(Self, Zeroizing<String>)> {
        let mnemonic = Mnemonic::new(MnemonicType::Words12, Language::English);

        let phrase = mnemonic.phrase().to_owned();

        let keypair =
            Keypair::from_seed_phrase_and_passphrase(&phrase, &passphrase.unwrap_or_default())
                .map_err(|error| {
                    AtollWalletError::UnableToConvertMnemonicToKeypair(error.to_string())
                })?;

        Ok((Self(keypair), Zeroizing::new(phrase)))
    }

    pub(crate) fn expose(&self) -> &Keypair {
        &self.0
    }
}

#[derive(Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash, Clone)]
pub struct SolanaWalletAccount<'wa> {
    address: Cow<'wa, str>,
    public_key: Byte32Array,
    icon: Option<WalletStandardIcon>,
    label: Option<&'wa str>,
    mainnet_enabled: bool,
    clusters: Cow<'wa, [SolanaCluster]>,
}

impl<'wa> SolanaWalletAccount<'wa> {
    pub fn new(public_key: Byte32Array) -> Self {
        let address = Cow::Owned(bs58::encode(&public_key).into_string());

        Self {
            address,
            public_key,
            clusters: Cow::Borrowed(&[
                SolanaCluster::Mainnet,
                SolanaCluster::Testnet,
                SolanaCluster::Devnet,
                SolanaCluster::Localnet,
            ]),
            mainnet_enabled: true,
            ..Default::default()
        }
    }

    pub fn allow_mainnet(&mut self) -> &mut Self {
        self.mainnet_enabled = true;
        self
    }

    pub fn disable_mainnet(&mut self) -> &mut Self {
        self.mainnet_enabled = false;
        self
    }

    pub fn set_icon(mut self, icon: WalletStandardIcon) -> Self {
        self.icon.replace(icon);

        self
    }

    pub fn set_label(mut self, label: &'wa str) -> Self {
        self.label.replace(label);

        self
    }

    pub fn chains(&'wa self) -> Cow<'wa, [&'wa str]> {
        let chains = self
            .clusters
            .iter()
            .map(|cluster| cluster.chain())
            .collect::<Vec<&str>>();

        Cow::Owned(chains)
    }

    pub fn features(&'wa self) -> Cow<'wa, [&'wa str]> {
        let mut features = Vec::<&str>::default();

        if let Some(value) = self.sign_in() {
            features.push(value)
        }

        features.extend_from_slice(&[
            self.sign_message(),
            self.sign_transaction(),
            self.sign_and_send_transaction(),
        ]);

        Cow::Owned(features)
    }

    pub fn to_js_value_object(&self) -> JsValue {
        let wallet_account_object = Reflection::new_object();

        let public_key_js_value = Reflection::new_uint8_array(&self.public_key);
        let chains_js_value = Reflection::new_str_array(self.chains().as_ref());
        let features_js_value = Reflection::new_str_array(self.features().as_ref());

        wallet_account_object
            .set_object_secure("address", &self.address.as_ref().into())
            .set_object_secure("publicKey", &public_key_js_value)
            .set_object_secure("chains", &chains_js_value)
            .set_object_secure("features", &features_js_value)
            .set_object_secure(
                "icon",
                &self.icon.map(|value| value.base64().to_string()).into(),
            )
            .set_object_secure("label", &self.label().into());

        wallet_account_object.take()
    }
}

impl<'wa> WalletAccount for SolanaWalletAccount<'wa> {
    fn address(&self) -> &str {
        &self.address
    }

    fn public_key(&self) -> &Byte32Array {
        &self.public_key
    }

    fn icon(&self) -> Option<WalletStandardIcon> {
        self.icon
    }

    fn label(&self) -> Option<&str> {
        self.label
    }
}

impl<'wa> ClusterEnabled for SolanaWalletAccount<'wa> {
    fn mainnet(&self) -> bool {
        self.mainnet_enabled
    }

    fn testnet(&self) -> bool {
        true
    }

    fn devnet(&self) -> bool {
        true
    }

    fn localnet(&self) -> bool {
        true
    }
}

impl fmt::Display for SolanaWalletAccount<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("WalletAccount")
            .field("address", &self.address)
            .field("public_key", &bs58::encode(&self.public_key).into_string())
            .field("mainnet_enabled", &self.mainnet_enabled)
            .field("icon", &self.icon)
            .field("label", &self.label)
            .finish()
    }
}
