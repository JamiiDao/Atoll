use wallet_standard_base::{SemverVersion, Wallet, WalletStandardIcon};

use crate::SolanaWalletAccount;

/// Wallet information without any browser function calls for `wallet-adapter` standard operations
#[derive(Clone, Default, PartialEq, Eq)]
pub struct SolanaWallet<'wa> {
    label: &'static str,
    version: SemverVersion,
    icon: Option<WalletStandardIcon>,
    accounts: Vec<SolanaWalletAccount<'wa>>,
    mainnet_enabled: bool,
}

impl<'wa> SolanaWallet<'wa> {
    /// Instantiate a new [WalletData]
    pub fn new(label: &'static str) -> Self {
        Self {
            label,
            ..Default::default()
        }
    }

    /// Set the [Semver version](SemverVersion) of the wallet
    pub fn set_version(mut self, version: SemverVersion) -> Self {
        self.version = version;

        self
    }

    /// Set the icon of the wallet.
    /// Should be in Base64 URL web format `data:image/${'svg+xml' | 'webp' | 'png' | 'gif'};base64,${string}`
    pub fn set_icon(mut self, icon: WalletStandardIcon) -> Self {
        self.icon.replace(icon);

        self
    }

    /// Add a [Wallet account](WalletAccountData) data
    pub fn add_account(mut self, account: SolanaWalletAccount<'wa>) -> Self {
        self.accounts.push(account);

        self
    }

    /// Add multiple [Wallet account](WalletAccountData) data(s)
    pub fn add_accounts(mut self, accounts: &[SolanaWalletAccount<'wa>]) -> Self {
        self.accounts.extend_from_slice(accounts);

        self
    }

    /// Replace all [Wallet account](WalletAccountData) data
    pub fn replace_accounts(mut self, accounts: Vec<SolanaWalletAccount<'wa>>) -> Self {
        self.accounts = accounts;

        self
    }

    /// Add a [Cluster]
    pub fn enable_mainnet(&'wa mut self) -> &'wa mut Self {
        self.mainnet_enabled = true;

        self
    }

    /// Add a [Cluster]
    pub fn disable_mainnet(&'wa mut self) -> &'wa mut Self {
        self.mainnet_enabled = false;

        self
    }

    /// Get the accounts provided by the wallet
    pub fn accounts(&self) -> &[SolanaWalletAccount<'wa>] {
        &self.accounts
    }
}

impl<'wa> Wallet for SolanaWallet<'wa> {
    fn label(&self) -> &str {
        self.label
    }

    fn version(&self) -> SemverVersion {
        self.version
    }

    fn icon(&self) -> Option<WalletStandardIcon> {
        self.icon
    }
}

impl<'wa> core::fmt::Debug for SolanaWallet<'wa> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Wallet")
            .field("name", &self.label)
            .field("version", &self.version)
            .field("icon", &self.icon)
            .field("accounts", &self.accounts)
            .field("mainnet_enabled", &self.mainnet_enabled)
            .finish()
    }
}

impl<'wa> PartialOrd for SolanaWallet<'wa> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl<'wa> Ord for SolanaWallet<'wa> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.label
            .as_bytes()
            .cmp(other.label.as_bytes())
            .then(self.version.cmp(&other.version))
    }
}

impl<'wa> core::hash::Hash for SolanaWallet<'wa> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.label.as_bytes().hash(state);
        self.version.hash(state);
    }
}
