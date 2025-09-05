function injectPageWallet() {
  const SigninInput = {
    // Optional EIP-4361 domain requesting the sign-in.
    domain: null, // string | null

    // Optional Solana Base58 address performing the sign-in.
    address: null, // string | null

    // Optional EIP-4361 Statement.
    statement: null, // string | null

    // Optional EIP-4361 URI.
    uri: null, // string | null

    // Optional EIP-4361 version.
    version: null, // string | null

    // Optional EIP-4361 Chain ID.
    // Possible values: "mainnet", "testnet", "devnet", "localnet", "solana:mainnet", "solana:testnet", "solana:devnet"
    chainId: null, // string | null

    // Optional EIP-4361 Nonce.
    nonce: null, // string | null

    // Optional ISO 8601 datetime string (issued at).
    issuedAt: null, // string | null (ISO 8601)

    // Optional ISO 8601 datetime string (expiration).
    expirationTime: null, // string | null (ISO 8601)

    // Optional ISO 8601 datetime string (not before).
    notBefore: null, // string | null (ISO 8601)

    // Optional EIP-4361 Request ID.
    requestId: null, // string | null

    // Optional EIP-4361 Resources.
    resources: [], // string[]
  };

  const iconData =
    "PHN2ZyBoZWlnaHQ9IjEyMy40NSIgdmlld0JveD0iMCAwIDEyMy40NSAxMjMuNDUiIHdpZHRoPSIxMjMuNDUiIHhtbG5zPSJodHRwOi8vd3d3LnczLm9yZy8yMDAwL3N2ZyI+PGNpcmNsZSBjeD0iNjEuNzIiIGN5PSI2MS43MyIgZmlsbD0iIzBmZiIgcj0iNTguNzc2IiBzdHJva2U9IiNmZjdmMmEiIHN0cm9rZS1saW5lY2FwPSJyb3VuZCIgc3Ryb2tlLWxpbmVqb2luPSJyb3VuZCIgc3Ryb2tlLXdpZHRoPSI1LjkiLz48L3N2Zz4=";

  const walletAccount = {
    address: "BtRL4ydcfGzRRZBUosHh6aQBg9DzLdNN6Nybhce6fqF3",
    publicKey: [
      161, 192, 255, 200, 114, 43, 197, 135, 210, 81, 184, 46, 253, 220, 248,
      75, 127, 99, 22, 137, 190, 13, 51, 164, 25, 226, 205, 215, 91, 235, 218,
      14,
    ],
    chains: ["solana:mainnet", "solana:devnet", "solana:testnet"],
    features: [
      "standard:connect",
      "standard:disconnect",
      "standard:events",
      "solana:signIn",
      "solana:signAndSendTransaction",
      "solana:signTransaction",
      "solana:signMessage",
    ],
    icon: undefined,
    label: undefined,
  };

  // Custom namespace
  const GhostNamespace = "ghost:";

  class GhostWallet {
    #listeners = {};
    #version = "1.0.0";
    #name = "Ghost";
    #icon = `data:image/svg+xml;base64,${iconData}`;
    #account = null;
    #ghost;
    #chains = ["solana:mainnet", "solana:devnet", "solana:testnet"];

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
        "standard:connect": {
          version: "1.0.0",
          connect: this.#connect,
        },
        "standard:disconnect": {
          version: "1.0.0",
          disconnect: this.#disconnect,
        },
        "standard:events": {
          version: "1.0.0",
          on: this.#on,
        },
        "solana:signIn": {
          version: "1.0.0",
          signIn: this.#signIn,
        },
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
        [GhostNamespace]: {
          ghost: this.#ghost,
        },
      };
    }

    get accounts() {
      return this.#account ? [this.#account] : [];
    }

    constructor(ghost) {
      if (new.target === GhostWallet) {
        Object.freeze(this);
      }

      // Fallback ghost so constructor doesn’t crash
      this.#ghost = ghost || {
        on: () => {}, // no-op event listeners
        connect: async () => {},
        disconnect: async () => {},
      };

      // If account was not connected before
      this.#ghost.on("connect", this.#connected, this);
      // If account is disconnected
      this.#ghost.on("disconnect", this.#disconnected, this);
      // If an account that had been connected before has now been reconnected
      this.#ghost.on("accountChanged", this.#reconnected, this);

      this.#connected();
    }

    #on = (event, listener) => {
      this.#listeners[event]?.push(listener) ||
        (this.#listeners[event] = [listener]);
      return () => this.#off(event, listener);
    };

    #emit(event, ...args) {
      this.#listeners[event]?.forEach((listener) => listener.apply(null, args));
    }

    #off(event, listener) {
      this.#listeners[event] = this.#listeners[event]?.filter(
        (existingListener) => listener !== existingListener
      );
    }

    #connected = () => {
      if (this.#account) {
        this.#emit("connected", { accounts: this.accounts });
      }
    };

    #disconnected = () => {
      if (this.#account) {
        this.#account = null;
        this.#emit("disconnected", { accounts: this.accounts });
      }
    };

    #reconnected = () => {
      if (this.#ghost.publicKey) {
        this.#connected();
      } else {
        this.#disconnected();
      }
    };

    #connect = async ({ silent } = {}) => {
      if (!this.#account) {
        await this.#ghost.connect(silent ? { onlyIfTrusted: true } : undefined);

        // Forcefully set account here
        this.#account = walletAccount;
      }

      this.#connected();

      return { accounts: this.accounts };
    };

    #disconnect = async () => {
      await this.#ghost.disconnect();
    };

    #signAndSendTransaction = async (...inputs) => {
      console.log("SIGN & SEND TX", ...inputs);

      const signedTx = {
        signature: new Uint8Array([
          180, 112, 2, 105, 215, 206, 164, 24, 53, 149, 74, 176, 11, 140, 1, 89,
          86, 240, 200, 228, 61, 67, 223, 197, 182, 245, 150, 152, 243, 192,
          122, 47, 253, 129, 122, 49, 206, 217, 224, 209, 248, 229, 60, 59, 172,
          95, 165, 249, 43, 189, 198, 126, 213, 83, 124, 159, 165, 187, 202,
          173, 182, 138, 57, 9,
        ]),
      };

      return [signedTx];
    };

    #signTransaction = async (...inputs) => {
      console.log("SIGN TX", ...inputs);

      const signedTx = {
        signedTransaction: new Uint8Array([
          1, 229, 133, 78, 137, 7, 52, 222, 43, 16, 149, 159, 32, 255, 253, 85,
          221, 138, 22, 244, 148, 102, 127, 17, 218, 159, 80, 121, 140, 127, 69,
          82, 139, 138, 134, 233, 26, 201, 198, 215, 91, 119, 129, 97, 2, 78,
          163, 78, 43, 140, 236, 34, 150, 71, 233, 231, 198, 4, 242, 94, 195,
          160, 104, 141, 2, 1, 0, 2, 4, 161, 192, 255, 200, 114, 43, 197, 135,
          210, 81, 184, 46, 253, 220, 248, 75, 127, 99, 22, 137, 190, 13, 51,
          164, 25, 226, 205, 215, 91, 235, 218, 14, 109, 242, 216, 61, 89, 111,
          107, 136, 182, 152, 92, 111, 23, 143, 94, 66, 80, 99, 6, 214, 71, 85,
          94, 223, 56, 29, 162, 145, 173, 168, 226, 100, 0, 0, 0, 0, 0, 0, 0, 0,
          0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
          0, 0, 0, 0, 3, 6, 70, 111, 229, 33, 23, 50, 255, 236, 173, 186, 114,
          195, 155, 231, 188, 140, 229, 187, 197, 247, 18, 107, 44, 67, 155, 58,
          64, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
          0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 3, 0, 9, 3, 216, 184, 5, 0, 0, 0,
          0, 0, 3, 0, 5, 2, 64, 13, 3, 0, 2, 2, 0, 1, 12, 2, 0, 0, 0, 0, 101,
          205, 29, 0, 0, 0, 0,
        ]),
      };

      return [signedTx];
    };

    #signMessage = async (...inputs) => {
      console.log("SIGN MESSAGE INPUTS", ...inputs);

      const signedMessageOutput = {
        signedMessage: new Uint8Array([
          83, 111, 108, 97, 110, 97, 32, 70, 111, 117, 110, 100, 97, 116, 105,
          111, 110, 32, 105, 115, 32, 97, 119, 101, 115, 111, 109, 101, 33,
        ]),
        signature: new Uint8Array([
          180, 112, 2, 105, 215, 206, 164, 24, 53, 149, 74, 176, 11, 140, 1, 89,
          86, 240, 200, 228, 61, 67, 223, 197, 182, 245, 150, 152, 243, 192,
          122, 47, 253, 129, 122, 49, 206, 217, 224, 209, 248, 229, 60, 59, 172,
          95, 165, 249, 43, 189, 198, 126, 213, 83, 124, 159, 165, 187, 202,
          173, 182, 138, 57, 9,
        ]),
      };

      return [signedMessageOutput];
    };

    #signIn = async (...inputs) => {
      console.log("SIWS", ...inputs);

      /*
      SIWS PARSER 127.0.0.1:8081 wants you to sign in with your Solana account:
BtRL4ydcfGzRRZBUosHh6aQBg9DzLdNN6Nybhce6fqF3

Community: JamiiDAOUSER ID: X48K48SESSION: 50c16142e314c8dea38e00effc113b41449868ea4aa9f2ac9fb589e4fe2646fb

Chain ID: solana:devnet
Nonce: 50c16142e314c8dea38e00effc113b41449868ea4aa9f2ac9fb589e4fe2646fb
*/

      const signInOutputExample = {
        account: walletAccount,
        signedMessage: new Uint8Array([
          49, 50, 55, 46, 48, 46, 48, 46, 49, 58, 56, 48, 56, 49, 32, 119, 97,
          110, 116, 115, 32, 121, 111, 117, 32, 116, 111, 32, 115, 105, 103,
          110, 32, 105, 110, 32, 119, 105, 116, 104, 32, 121, 111, 117, 114, 32,
          83, 111, 108, 97, 110, 97, 32, 97, 99, 99, 111, 117, 110, 116, 58, 10,
          66, 116, 82, 76, 52, 121, 100, 99, 102, 71, 122, 82, 82, 90, 66, 85,
          111, 115, 72, 104, 54, 97, 81, 66, 103, 57, 68, 122, 76, 100, 78, 78,
          54, 78, 121, 98, 104, 99, 101, 54, 102, 113, 70, 51, 10, 10, 67, 111,
          109, 109, 117, 110, 105, 116, 121, 58, 32, 74, 97, 109, 105, 105, 68,
          65, 79, 85, 83, 69, 82, 32, 73, 68, 58, 32, 88, 52, 56, 75, 52, 56,
          83, 69, 83, 83, 73, 79, 78, 58, 32, 99, 102, 57, 100, 98, 55, 50, 100,
          53, 55, 51, 100, 99, 48, 100, 56, 49, 98, 53, 97, 49, 50, 56, 56, 48,
          48, 49, 51, 52, 101, 56, 54, 99, 50, 51, 56, 55, 101, 99, 97, 54, 55,
          100, 102, 56, 99, 101, 97, 49, 100, 100, 51, 97, 101, 55, 97, 56, 101,
          50, 55, 50, 52, 53, 101, 10, 10, 67, 104, 97, 105, 110, 32, 73, 68,
          58, 32, 115, 111, 108, 97, 110, 97, 58, 100, 101, 118, 110, 101, 116,
          10, 78, 111, 110, 99, 101, 58, 32, 99, 102, 57, 100, 98, 55, 50, 100,
          53, 55, 51, 100, 99, 48, 100, 56, 49, 98, 53, 97, 49, 50, 56, 56, 48,
          48, 49, 51, 52, 101, 56, 54, 99, 50, 51, 56, 55, 101, 99, 97, 54, 55,
          100, 102, 56, 99, 101, 97, 49, 100, 100, 51, 97, 101, 55, 97, 56, 101,
          50, 55, 50, 52, 53, 101,
        ]),
        signature: new Uint8Array([
          224, 61, 134, 1, 96, 1, 252, 11, 203, 38, 136, 186, 32, 245, 142, 24,
          202, 87, 46, 0, 171, 38, 234, 205, 209, 216, 219, 164, 9, 225, 199,
          153, 26, 111, 57, 76, 132, 84, 184, 119, 37, 24, 70, 20, 118, 86, 66,
          81, 40, 68, 129, 253, 35, 117, 222, 85, 48, 249, 39, 95, 136, 114,
          171, 12,
        ]),
      };

      return [signInOutputExample];
    };
  }

  const wallet = new GhostWallet();
  console.log(wallet);

  const WALLET_REGISTER_EVENT = "wallet-standard:register-wallet";
  const APP_READY_EVENT = "wallet-standard:app-ready";

  // Register the wallet when the app signals it's ready
  window.addEventListener(APP_READY_EVENT, () => {
    const callback = (api) => {
      api.register(wallet);
    };

    window.dispatchEvent(
      new CustomEvent(WALLET_REGISTER_EVENT, {
        detail: callback,
      })
    );

    console.log(window.document.title);

    console.log("PAGE: Wallet registered");
  });
}

const script = document.createElement("script");
script.textContent = `(${injectPageWallet.toString()})();`;
(document.head || document.documentElement).appendChild(script);
script.remove();
