use wallet_standard_base::Cluster;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Default, Hash)]
pub enum BitcoinCluster {
    Mainnet(&'static str),
    Testnet,
    #[default]
    Regtest,
}

impl Cluster for BitcoinCluster {
    fn identifier(&self) -> &str {
        match self {
            Self::Mainnet(_) => "mainnet",
            Self::Testnet => "testnet",
            Self::Regtest => "regtest",
        }
    }

    fn chain(&self) -> &str {
        match self {
            Self::Mainnet(_) => "bitcoin:mainnet",
            Self::Testnet => "bitcoin:testnet",
            Self::Regtest => "bitcoin:regtest",
        }
    }

    fn endpoint(&self) -> &str {
        todo!()
    }

    fn chains(&self) -> &'static [&'static str] {
        todo!()
    }
}
