# Atoll
Reference implementation for a browser extension wallet that aim to support wasm wallet-adapter implementations. It shows how to inject a reference wallet to the DOM and communicate between content scripts and background tasks implementing wasm. Adding Bitcoin gives tests for other blockchains than Solana. It is still under develpment so Bitcoin features are missing. It implements:

- standard:connect
- standard:disconnect
- standard:event
- solana:signMessage
- solana:signIn
- solana:signTransaction
- solana:signAndSendTransaction

- bitcoin:signMessage
- bitcoin:signIn
- bitcoin:signTransaction
- bitcoin:signAndSendTransaction

The aim of this project is to provide a refrence implementation for wasm browser extension wallet and to test accuracy of wallet-adapter libraries like the [Rust Wallet-Adapter](https://github.com/JamiiDao/SolanaWalletAdapter).
