use std::{cell::RefCell, rc::Rc};

use wallet_standard_base::{SemverVersion, WalletStandardIcon, WindowEvent};
use wasm_bindgen::{
    JsCast, JsValue,
    prelude::{Closure, wasm_bindgen},
};
use web_sys::{CustomEvent, CustomEventInit, EventTarget, console, js_sys};

use crate::{ICON, Reflection, SolanaConstants, SolanaWalletAccount, WALLET_NAME};

#[wasm_bindgen]
pub fn get_injected_wallet() {
    let window = web_sys::window().expect("Window was not found or could not be detected");
    InjectedWallet::inject_wallet_events(window);
}

pub struct InjectedWallet {
    accounts: Rc<RefCell<Option<SolanaWalletAccount<'static>>>>,
    reflect: Reflection,
    chains: &'static [&'static str],
    icon: WalletStandardIcon,
    name: &'static str,
}

impl InjectedWallet {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn to_object(self) -> JsValue {
        self.name().version().icon().chains().accounts().features();

        self.reflect.take()
    }

    pub fn name(&self) -> &Self {
        self.reflect.set_object_secure("name", &self.name.into());

        self
    }

    pub fn version(&self) -> &Self {
        let version = SemverVersion::new().set_major(1).set_minor(0).set_patch(0);

        FieldWithGetter::new("version", version.to_string().into()).build(&self.reflect);

        self
    }

    pub fn icon(&self) -> &Self {
        let icon = self.icon.base64();

        self.reflect
            .set_object_secure("icon", &icon.as_ref().into());

        self
    }

    pub fn chains(&self) -> &Self {
        let chains = js_sys::Array::new();

        self.chains.iter().for_each(|chain| {
            chains.push(&(*chain).into());
        });

        self.reflect.set_object_secure("chains", &chains);

        self
    }

    pub fn accounts(&self) -> &Self {
        let accounts = js_sys::Array::new();

        if let Some(account) = self.accounts.borrow().as_ref() {
            accounts.push(&account.to_js_value_object());
        }

        self.reflect.set_object_secure("accounts", &accounts);

        self
    }

    pub fn features(&self) -> &Self {
        let features_object = Features::new()
            .standard_connect()
            .standard_disconnect()
            .standard_events()
            .sign_in()
            .sign_message()
            .sign_message()
            .sign_transaction()
            .sign_and_send_transaction()
            .0
            .take();

        self.reflect.set_object_secure("features", &features_object);

        self
    }

    pub fn inject_wallet_events(window: web_sys::Window) {
        let target: EventTarget = window
            .clone()
            .dyn_into()
            .expect("Unable to get the `EventTarget` from the window");

        let injected_wallet = InjectedWallet::new();
        let injected_wallet_object = injected_wallet.to_object();

        console::log_2(&"Entire wallet".into(), &injected_wallet_object);

        let closure = Closure::wrap(Box::new(move |_event: web_sys::Event| {
            let window = window.clone();

            let injected_wallet_object = injected_wallet_object.clone();

            // Build the callback: (api) => api.register(wallet)
            let callback = Closure::wrap(Box::new(move |api: JsValue| {
                let register_fn = js_sys::Reflect::get(&api, &JsValue::from_str("register"))
                    .expect("Unable to get the register function")
                    .dyn_into::<js_sys::Function>()
                    .expect("`register` is not a function");
                let _ = register_fn.call1(&api, &injected_wallet_object);
            }) as Box<dyn FnMut(JsValue)>);

            // Create CustomEventInit with { detail: callback }
            let event_init = CustomEventInit::new();
            event_init.set_detail(&callback.as_ref().into());

            // Dispatch event
            let custom_event = CustomEvent::new_with_event_init_dict(
                WindowEvent::Register.event_identifier(),
                &event_init,
            )
            .expect("Unable to set CustomEvent");
            window
                .dispatch_event(&custom_event)
                .expect("Unable to dispatch register event");

            callback.forget();
        }) as Box<dyn FnMut(_)>);

        target
            .add_event_listener_with_callback(
                WindowEvent::AppReady.event_identifier(),
                closure.as_ref().unchecked_ref(),
            )
            .expect("Unable to add event listener for `appready`");
        closure.forget();
    }
}

impl Default for InjectedWallet {
    fn default() -> Self {
        Self {
            icon: WalletStandardIcon::new_svg(ICON),
            name: WALLET_NAME,
            accounts: Rc::new(RefCell::new(Option::default())),
            reflect: Reflection::new_object(),
            chains: &[
                SolanaConstants::MAINNET_CHAIN,
                SolanaConstants::TESTNET_CHAIN,
                SolanaConstants::DEVNET_CHAIN,
                SolanaConstants::LOCALNET_CHAIN,
            ],
        }
    }
}

pub struct Features(Reflection);

impl Features {
    fn new() -> Self {
        Self(Reflection::new_object())
    }

    fn set_version(&self, target_object: &Reflection) {
        target_object.set_object_secure(
            "version",
            &SemverVersion::new()
                .set_major(1)
                .set_minor(0)
                .set_patch(0)
                .to_string()
                .into(),
        );
    }

    fn standard_connect(self) -> Self {
        let connect_function = Closure::wrap(Box::new(move || -> JsValue {
            // TODO: Send a message to the background
            console::log_1(&"standard:connect FUNCTION".into());

            JsValue::NULL
        }) as Box<dyn Fn() -> JsValue>);

        let connect_object = Reflection::new_object();
        self.set_version(&connect_object);
        connect_object.set_object_secure("connect", connect_function.as_ref().unchecked_ref());

        self.0
            .set_object_secure(SolanaConstants::STANDARD_CONNECT, &connect_object.take());

        connect_function.forget();

        self
    }

    fn standard_disconnect(self) -> Self {
        let disconnect_function = Closure::wrap(Box::new(move || -> JsValue {
            // TODO: Send a message to the background

            console::log_1(&"standard:disconnect FUNCTION".into());

            JsValue::NULL
        }) as Box<dyn Fn() -> JsValue>);

        let disconnect_object = Reflection::new_object();
        self.set_version(&disconnect_object);
        disconnect_object
            .set_object_secure("disconnect", disconnect_function.as_ref().unchecked_ref());

        self.0.set_object_secure(
            SolanaConstants::STANDARD_DISCONNECT,
            &disconnect_object.take(),
        );

        disconnect_function.forget();

        self
    }

    fn standard_events(self) -> Self {
        let on_function =
            Closure::wrap(Box::new(move |event: JsValue, listener: js_sys::Function| {
                web_sys::console::log_1(&"standard:events > on called!".into());
                web_sys::console::log_2(&event, &listener);
            }) as Box<dyn FnMut(JsValue, js_sys::Function)>);

        let events_object = Reflection::new_object();
        self.set_version(&events_object);
        events_object.set_object_secure("on", on_function.as_ref().unchecked_ref());

        self.0
            .set_object_secure(SolanaConstants::STANDARD_EVENTS, &events_object.take());

        on_function.forget();

        self
    }

    fn sign_in(self) -> Self {
        let sign_in_fn = Closure::wrap(Box::new(move || -> JsValue {
            // TODO: Send a message to the background

            console::log_1(&"signIn FUNCTION".into());

            JsValue::NULL
        }) as Box<dyn Fn() -> JsValue>);

        let sign_in_object = Reflection::new_object();
        self.set_version(&sign_in_object);
        sign_in_object.set_object_secure("signIn", sign_in_fn.as_ref().unchecked_ref());

        self.0
            .set_object_secure(SolanaConstants::SIGN_IN, &sign_in_object.take());

        sign_in_fn.forget();

        self
    }

    fn sign_message(self) -> Self {
        let sign_message_fn = Closure::wrap(Box::new(move || -> JsValue {
            // TODO: Send a message to the background

            console::log_1(&"signMessage FUNCTION".into());

            JsValue::NULL
        }) as Box<dyn Fn() -> JsValue>);

        let sign_message_object = Reflection::new_object();
        self.set_version(&sign_message_object);
        sign_message_object
            .set_object_secure("signMessage", sign_message_fn.as_ref().unchecked_ref());

        self.0
            .set_object_secure(SolanaConstants::SIGN_MESSAGE, &sign_message_object.take());

        sign_message_fn.forget();

        self
    }

    fn sign_transaction(self) -> Self {
        let sign_transaction_fn = Closure::wrap(Box::new(move || -> JsValue {
            // TODO: Send a message to the background

            console::log_1(&"signTransaction FUNCTION".into());

            JsValue::NULL
        }) as Box<dyn Fn() -> JsValue>);

        let sign_transaction_object = Reflection::new_object();
        self.set_version(&sign_transaction_object);
        sign_transaction_object.set_object_secure(
            "signTransaction",
            sign_transaction_fn.as_ref().unchecked_ref(),
        );

        self.0.set_object_secure(
            SolanaConstants::SIGN_TRANSACTION,
            &sign_transaction_object.take(),
        );

        sign_transaction_fn.forget();

        self
    }

    fn sign_and_send_transaction(self) -> Self {
        let sign_and_send_transaction_fn = Closure::wrap(Box::new(move || -> JsValue {
            // TODO: Send a message to the background

            console::log_1(&"signAndSendTransaction FUNCTION".into());

            JsValue::NULL
        }) as Box<dyn Fn() -> JsValue>);

        let sign_and_send_transaction_object = Reflection::new_object();
        self.set_version(&sign_and_send_transaction_object);
        sign_and_send_transaction_object.set_object_secure(
            "signAndSendTransaction",
            sign_and_send_transaction_fn.as_ref().unchecked_ref(),
        );

        self.0.set_object_secure(
            SolanaConstants::SIGN_AND_SEND_TRANSACTION,
            &sign_and_send_transaction_object.take(),
        );

        sign_and_send_transaction_fn.forget();

        self
    }
}

struct FieldWithGetter {
    key: String,
    value: JsValue,
}

impl FieldWithGetter {
    pub fn new(key: &str, value: JsValue) -> Self {
        Self {
            key: key.to_string(),
            value,
        }
    }

    pub fn build(&self, wallet_object: &Reflection) {
        let value = self.value.clone();

        let version_getter = Closure::wrap(
            Box::new(move || -> JsValue { value.clone() }) as Box<dyn Fn() -> JsValue>
        );

        wallet_object.set_object_secure(self.key.as_str(), version_getter.as_ref().unchecked_ref());

        version_getter.forget();
    }
}
