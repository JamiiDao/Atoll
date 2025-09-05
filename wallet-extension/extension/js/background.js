// const walletAccount = {
//   address: "BtRL4ydcfGzRRZBUosHh6aQBg9DzLdNN6Nybhce6fqF3",
//   publicKey: new Uint8Array([
//     161, 192, 255, 200, 114, 43, 197, 135, 210, 81, 184, 46, 253, 220, 248, 75,
//     127, 99, 22, 137, 190, 13, 51, 164, 25, 226, 205, 215, 91, 235, 218, 14,
//   ]),
//   chains: ["solana:mainnet", "solana:devnet", "solana:testnet"],
//   features: [
//     "standard:connect",
//     "standard:disconnect",
//     "standard:events",
//     "solana:signIn",
//     "solana:signAndSendTransaction",
//     "solana:signTransaction",
//     "solana:signMessage",
//   ],
//   icon: undefined,
//   label: undefined,
// };

// (async () => {
//   const extension = typeof browser !== "undefined" ? browser : chrome;

//   extension.runtime.onMessage.addListener((message, sender, sendResponse) => {
//     console.log("Got message in background:", message);

//     setTimeout(() => {
//       sendResponse(walletAccount);
//     }, 500);

//     return true; // required since response is async
//   });
// })();

(async () => {
  const extension = typeof browser !== "undefined" ? browser : chrome;

  const wasm = await import(
    extension.runtime.getURL("js/wasm/atoll_wallet_webextension.js")
  );

  await wasm.default(); //Initialize first

  await wasm.app(extension);
})();
