use std::collections::HashMap;

use crate::{AtollWalletError, AtollWalletResult, SolanaAccountKeypair};

pub struct App {
    active: blake3::Hash,
    keypairs: HashMap<blake3::Hash, SolanaAccountKeypair>,
}

impl App {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn load_test_keypairs(&mut self) -> AtollWalletResult<&mut Self> {
        let keypair = SolanaAccountKeypair::new_test()?;
        let active_hash = Self::hash_active(&keypair);
        self.keypairs.insert(active_hash, keypair);
        self.set_active_with_hash(active_hash);

        Ok(self)
    }

    pub fn set_active(&mut self, keypair: &SolanaAccountKeypair) -> &mut Self {
        self.active = Self::hash_active(keypair);

        self
    }

    pub fn set_active_with_hash(&mut self, hash: blake3::Hash) -> &mut Self {
        self.active = hash;

        self
    }

    pub fn hash_active(keypair: &SolanaAccountKeypair) -> blake3::Hash {
        blake3::hash(&keypair.pubkey().to_bytes())
    }

    pub fn active_keypair(&mut self) -> AtollWalletResult<&mut SolanaAccountKeypair> {
        self.keypairs
            .get_mut(&self.active)
            .ok_or(AtollWalletError::UnauthorizedKeypairRequest)
    }
}

impl Default for App {
    fn default() -> Self {
        Self {
            active: blake3::hash(&[0u8; 32]),
            keypairs: HashMap::default(),
        }
    }
}
