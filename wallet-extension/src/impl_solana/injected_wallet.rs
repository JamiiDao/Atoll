use wallet_standard_base::{SemverVersion, WalletStandardIcon};
use wasm_bindgen::prelude::*;
use web_sys::js_sys;

use crate::{ICON, Reflection, SolanaConstants, WALLET_NAME};

#[wasm_bindgen]
pub fn get_injected_wallet_info() -> JsValue {
    InjectedWallet::new().to_object()
}

pub struct InjectedWallet {
    reflect: Reflection,
    chains: &'static [&'static str],
    icon: WalletStandardIcon,
    name: &'static str,
    version: SemverVersion,
    namespace: &'static str,
}

impl InjectedWallet {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn to_object(&self) -> JsValue {
        self.name().version().icon().chains();

        self.reflect.cloned()
    }

    pub fn name(&self) -> &Self {
        self.reflect.set_object_secure("name", &self.name.into());

        self
    }

    pub fn namespace(&self) -> &Self {
        self.reflect
            .set_object_secure("namespace", &self.namespace.into());

        self
    }

    pub fn version(&self) -> &Self {
        self.reflect
            .set_object_secure("version", &self.version.to_string().into());

        self
    }

    pub fn icon(&self) -> &Self {
        let icon = self.icon.base64();

        self.reflect
            .set_object_secure("icon", &icon.as_ref().into());

        self
    }

    pub fn chains(&self) -> &Self {
        let chains = js_sys::Array::new();

        self.chains.iter().for_each(|chain| {
            chains.push(&(*chain).into());
        });

        self.reflect.set_object_secure("chains", &chains);

        self
    }
}

impl Default for InjectedWallet {
    fn default() -> Self {
        Self {
            namespace: "atollWallet:",
            version: SemverVersion::new().set_major(1).set_minor(0).set_patch(0),
            icon: WalletStandardIcon::new_svg(ICON),
            name: WALLET_NAME,
            reflect: Reflection::new_object(),
            chains: &[
                SolanaConstants::MAINNET_CHAIN,
                SolanaConstants::TESTNET_CHAIN,
                SolanaConstants::DEVNET_CHAIN,
                SolanaConstants::LOCALNET_CHAIN,
            ],
        }
    }
}
