// (async () => {
// const extension = (typeof browser !== "undefined") ? browser : chrome;
//   const wasm = await import(
//     extension.runtime.getURL("js/wasm/atoll_wallet_webextension.js")
//   );
//   await wasm.default();

//   extension.runtime.onMessage.addListener((message, sender) => {
//     console.log("Got message:", message);

//     wasm.app_ready();

//     return Promise.resolve("Background received: " + message);
//   });
// })();
