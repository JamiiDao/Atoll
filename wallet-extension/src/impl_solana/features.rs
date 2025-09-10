use wallet_standard_base::StandardFeatures;

use crate::{SolanaConstants, SolanaWalletAccount};

impl StandardFeatures for SolanaWalletAccount<'_> {
    fn namespace(&self) -> &str {
        SolanaConstants::STANDARD_NAMESPACE
    }

    fn sign_in(&self) -> Option<&str> {
        Option::Some(SolanaConstants::SIGN_IN)
    }

    fn sign_message(&self) -> &str {
        SolanaConstants::SIGN_MESSAGE
    }

    fn sign_transaction(&self) -> &str {
        SolanaConstants::SIGN_TRANSACTION
    }

    fn sign_and_send_transaction(&self) -> &str {
        SolanaConstants::SIGN_AND_SEND_TRANSACTION
    }
}
