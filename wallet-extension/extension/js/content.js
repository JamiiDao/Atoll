(async () => {
  const wasm = await import(
    chrome.runtime.getURL("js/wasm/atoll_wallet_webextension.js")
  );

  await wasm.default();

  const walletInfo = wasm.get_injected_wallet_info();

  const STANDARD_CONNECT = "standard:connect";
  const RELAY_STANDARD_CONNECT = "relay:standard:connect";

  function injectPageWallet(walletInfo) {
    const STANDARD_CONNECT = "standard:connect";
    const RELAY_STANDARD_CONNECT = "relay:standard:connect";

    const WALLET_REGISTER_EVENT = "wallet-standard:register-wallet";
    const APP_READY_EVENT = "wallet-standard:app-ready";

    const AtollWalletNamespace = walletInfo.namespace;

    class AtollWallet {
      #listeners = {};
      #version = walletInfo.version;
      #name = walletInfo.name;
      #icon = walletInfo.icon;
      #account = null;
      #atollWallet;
      #chains = walletInfo.chains;

      get version() {
        return this.#version;
      }
      get name() {
        return this.#name;
      }
      get icon() {
        return this.#icon;
      }
      get chains() {
        return this.#chains;
      }

      get features() {
        return {
          "standard:connect": { version: "1.0.0", connect: this.#connect },
          "standard:disconnect": {
            version: "1.0.0",
            disconnect: this.#disconnect,
          },
          "standard:events": { version: "1.0.0", on: this.#on },
          "solana:signIn": { version: "1.0.0", signIn: this.#signIn },
          "solana:signAndSendTransaction": {
            version: "1.0.0",
            supportedTransactionVersions: ["legacy", 0],
            signAndSendTransaction: this.#signAndSendTransaction,
          },
          "solana:signTransaction": {
            version: "1.0.0",
            supportedTransactionVersions: ["legacy", 0],
            signTransaction: this.#signTransaction,
          },
          "solana:signMessage": {
            version: "1.0.0",
            signMessage: this.#signMessage,
          },
          [AtollWalletNamespace]: { ghost: this.#atollWallet },
        };
      }

      get accounts() {
        return this.#account ? [this.#account] : [];
      }

      constructor(ghost) {
        if (new.target === AtollWallet) {
          Object.freeze(this);
        }
        this.#atollWallet = ghost || {
          on: () => {},
          connect: async () => {},
          disconnect: async () => {},
        };
        this.#atollWallet.on("connect", this.#connected, this);
        this.#atollWallet.on("disconnect", this.#disconnected, this);
        this.#atollWallet.on("accountChanged", this.#reconnected, this);
        this.#connected();
      }

      #on = (event, listener) => {
        console.log("EVENT", event);
        console.log("EVENT_LISTENER", listener);
        this.#listeners[event]?.push(listener) ||
          (this.#listeners[event] = [listener]);
        return () => this.#off(event, listener);
      };
      #emit(event, ...args) {
        this.#listeners[event]?.forEach((listener) => {
          listener.apply(null, args);
        });
      }
      #off(event, listener) {
        this.#listeners[event] = this.#listeners[event]?.filter(
          (l) => l !== listener
        );
      }

      #connected = () => {
        if (this.#account) this.#emit("change", { accounts: this.accounts });
      };
      #disconnected = () => {
        console.log("called disconnect");
        if (this.#account) {
          this.#account = null;
          this.#emit("change", { accounts: this.accounts });
        }
      };
      #reconnected = () => {
        if (this.#atollWallet.publicKey) this.#connected();
        else this.#disconnected();
      };

      #connect = async ({ silent } = {}) => {
        if (!this.#account) {
          await this.#atollWallet.connect(
            silent ? { onlyIfTrusted: true } : undefined
          );

          const result = await new Promise((resolve, reject) => {
            const listener = (event) => {
              if (event.source !== window) return;
              if (event.data.type === RELAY_STANDARD_CONNECT) {
                window.removeEventListener("message", listener);

                if (event.data.failure) reject(new Error(event.data.failure));
                else resolve(event.data.success);
              }
            };
            window.addEventListener("message", listener);

            // Send request → content.js
            window.postMessage({ type: STANDARD_CONNECT, text: "" }, "*");
          });

          this.#account = result;
        }
        this.#connected();

        return { accounts: this.accounts };
      };

      #disconnect = async () => {
        await this.#atollWallet.disconnect();
      };
      #signAndSendTransaction = async (...inputs) => {
        console.log("SIGN & SEND TX", ...inputs);
        return [{ signature: new Uint8Array([1, 2, 3]) }];
      };
      #signTransaction = async (...inputs) => {
        console.log("SIGN TX", ...inputs);
        return [{ signedTransaction: new Uint8Array([4, 5, 6]) }];
      };
      #signMessage = async (...inputs) => {
        console.log("SIGN MSG", ...inputs);
        return [
          {
            signedMessage: new Uint8Array([7, 8, 9]),
            signature: new Uint8Array([10, 11, 12]),
          },
        ];
      };
      #signIn = async (...inputs) => {
        console.log("SIGN IN", ...inputs);
        return [
          {
            account: walletAccount,
            signedMessage: new Uint8Array([13, 14, 15]),
            signature: new Uint8Array([16, 17, 18]),
          },
        ];
      };
    }

    const wallet = new AtollWallet();

    // Register wallet when app is ready
    window.addEventListener(APP_READY_EVENT, () => {
      const callback = (api) => api.register(wallet);
      window.dispatchEvent(
        new CustomEvent(WALLET_REGISTER_EVENT, { detail: callback })
      );
    });
  }

  const script = document.createElement("script");
  script.textContent = `(${injectPageWallet.toString()})(${JSON.stringify(
    walletInfo
  )});`;
  (document.head || document.documentElement).appendChild(script);
  script.remove();

  // Content script side
  const extension = typeof browser !== "undefined" ? browser : chrome;

  window.addEventListener("message", (event) => {
    if (event.source !== window) return;
    if (event.data.type === STANDARD_CONNECT) {
      extension.runtime.sendMessage(
        { resource: event.data.type, data: window.location.origin },
        (response) => {
          console.log("Content script got response from background:", response);
          // Relay back under RELAY_STANDARD_CONNECT
          window.postMessage(
            {
              type: RELAY_STANDARD_CONNECT,
              success: response.success,
              failure: response.failure,
            },
            "*"
          );
        }
      );
    }
  });
})();
