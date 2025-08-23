import initWasmModule, { hello_wasm } from './wasm/atoll_wallet_webextension.js';

(async () => {
    await initWasmModule();
    hello_wasm();
})();