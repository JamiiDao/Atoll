(async () => {
  const extension = typeof browser !== "undefined" ? browser : chrome;

  // Load WASM
  const wasm = await import(
    extension.runtime.getURL("js/wasm/atoll_wallet_webextension.js")
  );
  await wasm.default();

  window.addEventListener("wallet-standard:app-ready", () => {
    // Communication between this and the extension background process
    // extension.runtime.sendMessage({ type: "APP_READY" });

    wasm.app_ready();
  });
})();
