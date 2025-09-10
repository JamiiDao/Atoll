use wasm_bindgen::JsValue;

pub type AtollWalletResult<T> = Result<T, AtollWalletError>;

#[derive(Debug, Clone, PartialEq, thiserror::Error)]
pub enum AtollWalletError {
    #[error("Expected a JsValue to be of type js_sys::Object but it is of type `{0}`")]
    JsValueIsNotAnObject(String),
    #[error("Unable to check the type of `{0:?}`")]
    UnableToCheckTypeOfJsValue(JsValue),
    #[error("Encountered an error when performing Bip39 operation. Error: `{0}`")]
    Bip39(String),
    #[error("Encountered an error when trying to convert a mnemonic to a keypair. Error: `{0}`")]
    UnableToConvertMnemonicToKeypair(String),
    #[error("extension.runtime is missing from extension object")]
    ExtensionRuntimeIsMissing,
    #[error("extension.runtime.onMessage is missing")]
    ExtensionRuntimeMessageIsMissing,
    #[error("extension.runtime.onMessage.addListener is missing in onMessage object")]
    ExtensionRuntimeMessageAddListenerIsMissing,
    #[error("{0}")]
    JsCast(String),
    #[error("{0}")]
    Input(String),
    #[error("The message `{0}` from `extension.runtime.onMessage.addListener` is not supported")]
    UnsupportedExtensionMessage(String),
    #[error(
        "The `data` field not found in `extension.runtime.onMessage.addListener.message` object."
    )]
    DataNotFoundInMessageObject,
    #[error(
        "The `resource` field not found in `extension.runtime.onMessage.addListener.message` object."
    )]
    ResourceNotFoundInMessageObject,
    #[error("The mnemonic provided to reconstruct the Solana Keypair is invalid")]
    UnableToRecoverSolanaKeypairFromMnemonic,
    #[error("A request was made to authorize a dapp but a keypair doesn't exist yet")]
    UnauthorizedKeypairRequest,
    #[error("The `{0}` timestamp is not a valid ISO8601 timestamp.")]
    InvalidIS08601Timestamp(String),
}

impl From<bip39::ErrorKind> for AtollWalletError {
    fn from(value: bip39::ErrorKind) -> Self {
        Self::Bip39(value.to_string())
    }
}

impl From<AtollWalletError> for JsValue {
    fn from(value: AtollWalletError) -> Self {
        JsValue::from_str(value.to_string().as_str())
    }
}
