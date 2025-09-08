(async () => {
  const extension = typeof browser !== "undefined" ? browser : chrome;

  const wasm = await import(
    extension.runtime.getURL("js/wasm/atoll_wallet_webextension.js")
  );

  await wasm.default(); //Initialize first

  await wasm.app(extension);
})();
