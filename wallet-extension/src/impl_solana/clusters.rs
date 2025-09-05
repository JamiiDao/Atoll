use wallet_standard_base::Cluster;

use crate::SolanaConstants;

#[derive(Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
pub enum SolanaCluster {
    /// Solana Mainnet cluster,  [https://api.mainnet-beta.solana.com](https://api.mainnet-beta.solana.com).
    Mainnet,
    /// Solana Testnet cluster, e.g. [https://api.testnet.solana.com](https://api.testnet.solana.com)
    Testnet,
    /// Solana Devnet cluster, e.g. [https://api.devnet.solana.com](https://api.devnet.solana.com)
    #[default]
    Devnet,
    /// Solana Localnet cluster, e.g. [http://localhost:8899](http://localhost:8899)
    Localnet,
}

impl Cluster for SolanaCluster {
    fn identifier(&self) -> &str {
        match self {
            Self::Mainnet => SolanaConstants::MAINNET_IDENTIFIER,
            Self::Testnet => SolanaConstants::TESTNET_IDENTIFIER,
            Self::Devnet => SolanaConstants::DEVNET_IDENTIFIER,
            Self::Localnet => SolanaConstants::LOCALNET_IDENTIFIER,
        }
    }

    fn chain(&self) -> &str {
        match self {
            Self::Mainnet => SolanaConstants::MAINNET_CHAIN,
            Self::Testnet => SolanaConstants::TESTNET_CHAIN,
            Self::Devnet => SolanaConstants::DEVNET_CHAIN,
            Self::Localnet => SolanaConstants::LOCALNET_CHAIN,
        }
    }

    fn endpoint(&self) -> &str {
        match self {
            Self::Mainnet => SolanaConstants::MAINNET_ENDPOINT,
            Self::Testnet => SolanaConstants::TESTNET_IDENTIFIER,
            Self::Devnet => SolanaConstants::DEVNET_ENDPOINT,
            Self::Localnet => SolanaConstants::LOCALNET_ENDPOINT,
        }
    }

    fn chains(&self) -> &'static [&'static str] {
        &[
            SolanaConstants::MAINNET_CHAIN,
            SolanaConstants::TESTNET_CHAIN,
            SolanaConstants::DEVNET_CHAIN,
            SolanaConstants::LOCALNET_CHAIN,
        ]
    }
}
