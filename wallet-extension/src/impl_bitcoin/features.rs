use wallet_standard_base::StandardFeatures;

/// [Bitcoin Extension](https://github.com/wallet-standard/wallet-standard/blob/master/extensions/bitcoin.md)
#[derive(Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
pub struct StandardFeaturesBitcoin {
    sign_in: bool,
}

impl StandardFeaturesBitcoin {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn enable_sign_in_feature(mut self) -> Self {
        self.sign_in = true;

        self
    }
}

impl StandardFeatures for StandardFeaturesBitcoin {
    fn namespace(&self) -> &str {
        "bitcoin"
    }

    fn connect(&self) -> &str {
        "bitcoin:connect"
    }

    fn sign_in(&self) -> Option<&str> {
        if self.sign_in {
            Option::Some("bitcoin:signIn")
        } else {
            Option::default()
        }
    }

    fn sign_message(&self) -> &str {
        "bitcoin:signMessage"
    }

    fn sign_transaction(&self) -> &str {
        "bitcoin:signTransaction"
    }

    fn sign_and_send_transaction(&self) -> &str {
        "bitcoin:signAndSendTransaction"
    }
}
