pub struct SolanaConstants;

impl SolanaConstants {
    pub const STANDARD_NAMESPACE: &str = "solana";
    pub const STANDARD_CONNECT: &str = "standard:connect";
    pub const STANDARD_DISCONNECT: &str = "standard:disconnect";
    pub const STANDARD_EVENTS: &str = "standard:events";

    pub const SIGN_IN: &str = "solana:signIn";
    pub const SIGN_MESSAGE: &str = "solana:signMessage";
    pub const SIGN_TRANSACTION: &str = "solana:signTransaction";
    pub const SIGN_AND_SEND_TRANSACTION: &str = "solana:signAndSendTransaction";

    pub const MAINNET_IDENTIFIER: &str = "mainnet";
    pub const TESTNET_IDENTIFIER: &str = "testnet";
    pub const DEVNET_IDENTIFIER: &str = "devnet";
    pub const LOCALNET_IDENTIFIER: &str = "localnet";

    pub const MAINNET_CHAIN: &str = "solana:mainnet";
    pub const TESTNET_CHAIN: &str = "solana:testnet";
    pub const DEVNET_CHAIN: &str = "solana:devnet";
    pub const LOCALNET_CHAIN: &str = "solana:localnet";

    pub const MAINNET_ENDPOINT: &str = "https://api.mainnet-beta.solana.com";
    pub const TESTNET_ENDPOINT: &str = "https://api.testnet.solana.com";
    pub const DEVNET_ENDPOINT: &str = "https://api.devnet.solana.com";
    pub const LOCALNET_ENDPOINT: &str = "http://localhost:8899";
}
