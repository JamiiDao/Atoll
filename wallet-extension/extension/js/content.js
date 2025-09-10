(async () => {
  const wasm = await import(
    chrome.runtime.getURL("js/wasm/atoll_wallet_webextension.js")
  );

  await wasm.default();

  const walletInfo = wasm.get_injected_wallet_info();

  const STANDARD_CONNECT = "standard:connect";
  const RELAY_STANDARD_CONNECT = "relay:standard:connect";

  const RELAY_SOLANA_SIGN_IN = "relay:solana:signIn";
  const SOLANA_SIGN_IN = "solana:signIn";

  const RELAY_SIGN_MESSAGE = "relay:solana:signMessage";
  const SOLANA_SIGN_MESSAGE = "solana:signMessage";

  const RELAY_SIGN_TRANSACTION = "relay:solana:signTransaction";
  const SOLANA_SIGN_TRANSACTION = "solana:signTransaction";

  const RELAY_SIGN_AND_SEND_TRANSACTION = "relay:solana:signAndSendTransaction";
  const SOLANA_SIGN_AND_SEND_TRANSACTION = "solana:signAndSendTransaction";

  function injectPageWallet(walletInfo) {
    const STANDARD_CONNECT = "standard:connect";
    const RELAY_STANDARD_CONNECT = "relay:standard:connect";

    const RELAY_SOLANA_SIGN_IN = "relay:solana:signIn";
    const SOLANA_SIGN_IN = "solana:signIn";

    const RELAY_SIGN_MESSAGE = "relay:solana:signMessage";
    const SOLANA_SIGN_MESSAGE = "solana:signMessage";

    const RELAY_SIGN_TRANSACTION = "relay:solana:signTransaction";
    const SOLANA_SIGN_TRANSACTION = "solana:signTransaction";

    const RELAY_SIGN_AND_SEND_TRANSACTION =
      "relay:solana:signAndSendTransaction";
    const SOLANA_SIGN_AND_SEND_TRANSACTION = "solana:signAndSendTransaction";

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
          const result = await sendRequest({
            requestType: STANDARD_CONNECT,
            relayType: RELAY_STANDARD_CONNECT,
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
        const result = await sendRequest({
          requestType: SOLANA_SIGN_AND_SEND_TRANSACTION,
          relayType: RELAY_SIGN_AND_SEND_TRANSACTION,
          requestData: inputs[0],
        });

        return result;
      };
      #signTransaction = async (...inputs) => {
        const result = await sendRequest({
          requestType: SOLANA_SIGN_TRANSACTION,
          relayType: RELAY_SIGN_TRANSACTION,
          requestData: inputs[0],
        });

        return result;
      };
      #signMessage = async (...inputs) => {
        const result = await sendRequest({
          requestType: SOLANA_SIGN_MESSAGE,
          relayType: RELAY_SIGN_MESSAGE,
          requestData: inputs[0],
        });

        return result;
      };
      #signIn = async (...inputs) => {
        const result = await sendRequest({
          requestType: SOLANA_SIGN_IN,
          relayType: RELAY_SOLANA_SIGN_IN,
          requestData: inputs[0],
        });

        return result;
      };
    }

    function sendRequest({ requestType, relayType, requestData }) {
      return new Promise((resolve, reject) => {
        const listener = (event) => {
          if (event.source !== window) return;
          if (event.data.type === relayType) {
            window.removeEventListener("message", listener);

            if (event.data.failure) {
              reject(new Error(event.data.failure));
            } else {
              resolve(event.data.success);
            }
          }
        };

        window.addEventListener("message", listener);

        // Fire request → content.js
        window.postMessage({ type: requestType, requestData }, "*");
      });
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

  setupRelayListener({
    requestType: STANDARD_CONNECT,
    relayType: RELAY_STANDARD_CONNECT,
    getData: () => window.location.origin,
  });

  setupRelayListener({
    requestType: SOLANA_SIGN_IN,
    relayType: RELAY_SOLANA_SIGN_IN,
    getData: (event) => event.data,
  });

  setupRelayListener({
    requestType: SOLANA_SIGN_MESSAGE,
    relayType: RELAY_SIGN_MESSAGE,
    getData: (event) => event.data,
  });

  setupRelayListener({
    requestType: SOLANA_SIGN_TRANSACTION,
    relayType: RELAY_SIGN_TRANSACTION,
    getData: (event) => event.data,
  });

  window.addEventListener("message", (event) => {
    if (event.source !== window) return;
    if (event.data.type === SOLANA_SIGN_AND_SEND_TRANSACTION) {
      extension.runtime.sendMessage(
        { resource: event.data.type, data: event.data },
        (response) => {
          console.log("Content script got response from background:", response);
          // Relay back under RELAY_STANDARD_CONNECT
          window.postMessage(
            {
              type: RELAY_SIGN_AND_SEND_TRANSACTION,
              success: response.success,
              failure: response.failure,
            },
            "*"
          );
        }
      );
    }
  });

  function setupRelayListener({ requestType, relayType, getData }) {
    window.addEventListener("message", (event) => {
      if (event.source !== window) return;
      if (event.data.type !== requestType) return;

      extension.runtime.sendMessage(
        { resource: requestType, data: getData(event) },
        (responsePromise) => {
          Promise.resolve(responsePromise)
            .then((success) => {
              window.postMessage({ type: relayType, success }, "*");
            })
            .catch((failure) => {
              window.postMessage({ type: relayType, failure }, "*");
            });
        }
      );
    });
  }
})();
