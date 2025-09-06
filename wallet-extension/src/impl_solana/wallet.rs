use std::{borrow::Cow, collections::HashMap};

use bip39::{Language, Mnemonic, MnemonicType};
use solana_keypair::Keypair;
use solana_pubkey::Pubkey;
use solana_seed_derivable::SeedDerivable;

use solana_signer::Signer;
use zeroize::Zeroizing;

use crate::{AtollWalletError, AtollWalletResult, SolanaWalletAccount};

const TEST_PASSPHRASE: &str = "quick brown fox";

const TEST_MNEMONIC: &str =
    "wrap kingdom punch clog kiss useless celery exist bulk catch share creek";

pub struct SolanaAccountKeypair {
    keypair: Keypair,
    active_dapps: HashMap<blake3::Hash, ActiveDapp>,
}

impl SolanaAccountKeypair {
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

    pub fn standard_connect<'wa>(&'wa mut self, uri: String) -> SolanaWalletAccount<'wa> {
        let (active_dapp, hash) = ActiveDapp::new(uri);
        self.active_dapps.insert(hash, active_dapp);

        let public_key = self.pubkey().to_bytes();

        SolanaWalletAccount::new(public_key)
    }

    pub fn get_wallet_account<'wa>(&'wa self) -> SolanaWalletAccount<'wa> {
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
