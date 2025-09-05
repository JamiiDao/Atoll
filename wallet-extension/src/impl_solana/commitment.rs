use wallet_standard_base::Commitment;

/// The commitment level of a Solana transaction.
///
/// Note that deprecated commitments are converted into supported commitments.
///
/// `recent` is parsed as `processed`
///
/// `single` and `singleGossip` are parsed as `confirmed`
///
/// `root` and `max` are parsed as `finalized`,
///
/// Note that invalid commitment will always be converted to `Commitment::default()`
#[derive(Debug, PartialEq, Eq, Default, PartialOrd, Ord, Clone, Copy, Hash)]
pub enum SolanaCommitment {
    /// A transaction has been validated and recorded in the blockchain by a single node
    Processed,
    /// A transaction has been validated and recorded by a majority of nodes in the Solana cluster.
    Confirmed,
    /// A has been included in a block that has been committed to the blockchain by the Solana cluster
    /// and is now irreversible.
    #[default]
    Finalized,
}

impl Commitment for SolanaCommitment {
    fn processed(&self) -> Self {
        Self::Processed
    }

    fn confirmed(&self) -> Self {
        Self::Confirmed
    }

    fn finalized(&self) -> Self {
        Self::Finalized
    }

    fn into(&self, commitment_str: &str) -> Self {
        match commitment_str {
            "processed" | "recent" => Self::Processed,
            "confirmed" | "single" | "singleGossip" => Self::Confirmed,
            "finalized" | "root" | "max" => Self::Finalized,
            _ => Self::default(),
        }
    }

    fn as_str(&self) -> &str {
        match self {
            Self::Processed => "processed",
            Self::Confirmed => "confirmed",
            Self::Finalized => "finalized",
        }
    }
}
