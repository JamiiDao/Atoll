use std::{collections::HashMap, sync::Arc};

use async_lock::RwLock;

use crate::{AtollWalletResult, SolanaAccountKeypair};

pub type ActiveHash = Arc<RwLock<blake3::Hash>>;
pub type KeypairOps = Arc<RwLock<HashMap<blake3::Hash, SolanaAccountKeypair>>>;

pub(crate) struct App {
    pub(crate) active: ActiveHash,
    pub(crate) keypairs: KeypairOps,
}

impl App {
    pub fn _new() -> Self {
        Self::default()
    }

    pub fn load_test_keypairs() -> AtollWalletResult<Self> {
        let keypair = SolanaAccountKeypair::new_test()?;
        let active = Self::hash_active(&keypair);
        let mut keypairs = HashMap::default();
        keypairs.insert(active, keypair);

        Ok(Self {
            active: Arc::new(RwLock::new(active)),
            keypairs: Arc::new(RwLock::new(keypairs)),
        })
    }

    pub fn _set_active(&mut self, keypair: &SolanaAccountKeypair) -> &mut Self {
        self.active = Arc::new(RwLock::new(Self::hash_active(keypair)));

        self
    }

    pub fn _set_active_with_hash(&mut self, hash: blake3::Hash) -> &mut Self {
        self.active = Arc::new(RwLock::new(hash));

        self
    }

    pub fn hash_active(keypair: &SolanaAccountKeypair) -> blake3::Hash {
        blake3::hash(&keypair.pubkey().to_bytes())
    }

    pub async fn _get_active_hash(&self) -> blake3::Hash {
        *self.active.read().await
    }

    pub async fn _keypair_op(
        &mut self,
    ) -> async_lock::RwLockWriteGuard<'_, HashMap<blake3::Hash, SolanaAccountKeypair>> {
        self.keypairs.write().await
    }
}

impl Default for App {
    fn default() -> Self {
        Self {
            active: Arc::new(RwLock::new(blake3::hash(&[0u8; 32]))),
            keypairs: Arc::new(RwLock::new(HashMap::default())),
        }
    }
}
