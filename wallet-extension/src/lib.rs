mod message_handler;
pub use message_handler::*;

mod impl_solana;
pub use impl_solana::*;

mod impl_bitcoin;
pub use impl_bitcoin::*;

mod errors;
pub use errors::*;

pub(crate) const WALLET_NAME: &str = "Atoll Wallet";

pub(crate) const ICON: &[u8] =
    include_bytes!(concat!(env!("CARGO_WORKSPACE_DIR"), "/atoll-logo.svg"));

pub(crate) const TEST_MNEMONIC: &str =
    "wrap kingdom punch clog kiss useless celery exist bulk catch share creek";

pub(crate) const TEST_PASSPHRASE: &str = "quick brown fox";
