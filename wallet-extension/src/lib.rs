mod message_handler;
pub use message_handler::*;

mod impl_solana;
pub use impl_solana::*;

mod impl_bitcoin;
pub use impl_bitcoin::*;

mod errors;
pub use errors::*;

mod app;
pub use app::*;

pub(crate) const WALLET_NAME: &str = "Atoll Wallet";

const ICON: &[u8] = include_bytes!(concat!(env!("CARGO_WORKSPACE_DIR"), "/atoll-logo.svg"));
