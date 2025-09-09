use std::time::{Duration, SystemTime, UNIX_EPOCH};

use wallet_standard_base::SignInInput;
use wasm_bindgen::JsValue;
use web_sys::js_sys::{self, Array, Uint8Array};

use crate::{
    App, AtollWalletError, AtollWalletResult, Reflection, SolanaCluster, SolanaConstants,
    app_console_log,
};

impl App {
    pub fn solana_sign_in(&mut self, data: JsValue) -> AtollWalletResult<JsValue> {
        let data = Reflection::new_object_from_js_value(data)?;
        let data = data.get_object(
            "requestData",
            AtollWalletError::JsCast("`requestData` not found in `message` object".to_string()),
        )?;

        app_console_log(SolanaConstants::SIGN_IN, &data);

        let mut input = SignInInputParser::new();

        let formatted_input = input.parse(data)?.format();

        let (wallet_account, signature) = self.active_keypair()?.sign_in(&formatted_input);

        let sign_in_output = Reflection::new_object();

        sign_in_output.set_object_secure("account", &wallet_account.to_js_value_object());

        let signed_message = Uint8Array::new_from_slice(formatted_input.as_bytes());
        sign_in_output.set_object_secure("signedMessage", &signed_message);

        let signature = Uint8Array::new_from_slice(&signature);
        sign_in_output.set_object_secure("signature", &signature);

        sign_in_output.set_object_secure("signatureType", &"ed25519".into());

        let output_array = Array::new();
        output_array.push(&sign_in_output.take());

        Ok(output_array.into())
    }
}

#[derive(Debug)]
struct SignInInputParser<'wa>(SignInInput<'wa>);

impl<'wa> SignInInputParser<'wa> {
    pub fn new() -> Self {
        Self(SignInInput::new())
    }

    pub fn parse(&'wa mut self, sign_in_input: JsValue) -> AtollWalletResult<&'wa mut Self> {
        let reflection = Reflection::new_object_from_js_value(sign_in_input)?;

        self.domain(&reflection)?
            .address(&reflection)?
            .statement(&reflection)?
            .uri(&reflection)?
            .version(&reflection)?
            .chain_id(&reflection)?
            .nonce(&reflection)?
            .issued_at(&reflection)?
            .expiration_time(&reflection)?
            .not_before(&reflection)?
            .request_id(&reflection)?
            .resources(&reflection)
    }

    /*
            ${domain} wants you to sign in with your Solana account:
    ${address}

    ${statement}

    URI: ${uri}
    Version: ${version}
    Chain ID: ${chain-id}
    Nonce: ${nonce}
    Issued At: ${issued-at}
    Expiration Time: ${expiration-time}
    Not Before: ${not-before}
    Request ID: ${request-id}
    Resources:
    - ${resources[0]}
    - ${resources[1]}
    ...
    - ${resources[n]}
     */
    pub fn format(&'wa self) -> String {
        let mut formatted = String::default();

        if let Some(domain) = self.0.domain().as_ref() {
            formatted.push_str(domain);
            formatted.push_str(" wants you to sign in with your Solana account: \n");
        }
        if let Some(address) = self.0.address().as_ref() {
            formatted.push_str(address);
            formatted.push_str("\n\n");
        }
        if let Some(statement) = self.0.statement().as_ref() {
            formatted.push_str(statement);
            formatted.push_str("\n\n");
        }
        if let Some(uri) = self.0.uri().as_ref() {
            formatted.push_str("URI: ");
            formatted.push_str(uri);
            formatted.push('\n');
        }
        if let Some(version) = self.0.version().as_ref() {
            formatted.push_str("Version: ");
            formatted.push_str(version);
            formatted.push('\n');
        }
        if let Some(chain_id) = self.0.chain_id().as_ref() {
            formatted.push_str("Chain ID: ");
            formatted.push_str(chain_id);
            formatted.push('\n');
        }
        if let Some(nonce) = self.0.nonce().as_ref() {
            formatted.push_str("Nonce: ");
            formatted.push_str(nonce);
            formatted.push('\n');
        }
        if let Some(issued_at) = self.0.issued_at().as_ref() {
            formatted.push_str("Issued At: ");
            formatted.push_str(issued_at);
            formatted.push('\n');
        }
        if let Some(expiration_time) = self.0.expiration_time().as_ref() {
            formatted.push_str("Expiration Time: ");
            formatted.push_str(expiration_time);
            formatted.push('\n');
        };
        if let Some(not_before) = self.0.not_before().as_ref() {
            formatted.push_str("Not Before: ");
            formatted.push_str(not_before);
            formatted.push('\n');
        }

        if let Some(request_id) = self.0.request_id().as_ref() {
            formatted.push_str("Request ID: ");
            formatted.push_str(request_id);
            formatted.push('\n');
        }

        if !self.0.resources().as_ref().is_empty() {
            formatted.push_str("Resources:\n");
            self.0.resources().iter().for_each(|resource| {
                formatted.push_str("- ");
                formatted.push_str(resource);
                formatted.push('\n');
            });
        }

        formatted
    }

    // TODO
    // pub fn checks(&self) -> AtollWalletResult<()> {
    //     self.outcome
    //         .statement()
    //         .map(|value| {
    //             if value.contains("\n") {
    //                 Err(AtollWalletError::Input("The `statement` value in `SignInInput` for Sign In With should not have any line breaks".to_string()))
    //             } else {
    //                 Ok(value)
    //             }
    //         })
    //         .transpose()?;

    //         self.outcome
    //             .nonce()
    //             .map(|value| {
    //                 if value.len() < 8 {
    //                     Err(AtollWalletError::Input("The `nonce` value in `SignInInput` for Sign In With should be 8 or more characters".to_string()))
    //                 } else {
    //                     Ok(value)
    //                 }
    //             })
    //             .transpose()?;

    //         self.outcome
    //             .issued_at()
    //             .map(|value| {
    // let system_time_issued = humantime::parse_rfc3339(value).or(Err(

    // ));

    //                 if value.len() < 8 {
    //                     Err(AtollWalletError::Input("The `nonce` value in `SignInInput` for Sign In With should be 8 or more characters".to_string()))
    //                 } else {
    //                     Ok(value)
    //                 }
    //             })
    //             .transpose()?;

    //         Ok(())
    //     }

    /// Fetches the time from [JavaScript Date Now](js_sys::Date::now()) .
    /// This is converted to [SystemTime]
    pub fn time_now() -> AtollWalletResult<SystemTime> {
        let date_now = js_sys::Date::now() as u64;

        UNIX_EPOCH
            .checked_add(Duration::from_millis(date_now))
            .ok_or(AtollWalletError::Input(
                "Invalid addition of time. UNIX_EPOCH.checked_add(js_sys::Date::now()".to_string(),
            ))
    }

    pub fn domain(&'wa mut self, reflection: &Reflection) -> AtollWalletResult<&'wa mut Self> {
        if let Some(domain_value) = reflection.reflect_string_or_undefined("domain") {
            self.0.set_domain(domain_value.as_str().trim());
        }

        Ok(self)
    }

    pub fn address(&mut self, reflection: &Reflection) -> AtollWalletResult<&mut Self> {
        if let Some(address) = reflection.reflect_string_or_undefined("address") {
            self.0
                .set_address(address.as_str().trim())
                .map_err(|error| AtollWalletError::Input(error.to_string()))?;
        }

        Ok(self)
    }

    pub fn statement(&mut self, reflection: &Reflection) -> AtollWalletResult<&mut Self> {
        if let Some(statement) = reflection.reflect_string_or_undefined("statement") {
            self.0.set_statement(statement.as_str().trim());
        }

        Ok(self)
    }

    pub fn uri(&mut self, reflection: &Reflection) -> AtollWalletResult<&mut Self> {
        if let Some(uri) = reflection.reflect_string_or_undefined("uri") {
            self.0.set_uri(uri.as_str().trim());
        }

        Ok(self)
    }

    pub fn version(&mut self, reflection: &Reflection) -> AtollWalletResult<&mut Self> {
        if let Some(version) = reflection.reflect_string_or_undefined("version") {
            self.0.set_version(version.as_str().trim());
        }

        Ok(self)
    }

    pub fn chain_id(&mut self, reflection: &Reflection) -> AtollWalletResult<&mut Self> {
        if let Some(chain_id) = reflection.reflect_string_or_undefined("chainId") {
            let cluster: SolanaCluster = chain_id.as_str().trim().into();

            self.0.set_chain_id(cluster);
        }

        Ok(self)
    }

    pub fn nonce(&mut self, reflection: &Reflection) -> AtollWalletResult<&mut Self> {
        if let Some(nonce) = reflection.reflect_string_or_undefined("nonce") {
            self.0
                .set_custom_nonce(nonce.as_str().trim())
                .map_err(|error| AtollWalletError::Input(error.to_string()))?;
        }

        Ok(self)
    }

    pub fn issued_at(&mut self, reflection: &Reflection) -> AtollWalletResult<&mut Self> {
        if let Some(issued_at) = reflection.reflect_string_or_undefined("issuedAt") {
            let issued_at = humantime::parse_rfc3339(&issued_at)
                .or(Err(AtollWalletError::InvalidIS08601Timestamp(issued_at)))?;
            self.0.set_issued_at(issued_at);
        }

        Ok(self)
    }

    pub fn expiration_time(&mut self, reflection: &Reflection) -> AtollWalletResult<&mut Self> {
        if let Some(expiration_time) = reflection.reflect_string_or_undefined("expirationTime") {
            let expiration_time = humantime::parse_rfc3339(&expiration_time).or(Err(
                AtollWalletError::InvalidIS08601Timestamp(expiration_time),
            ))?;
            self.0
                .set_expiration_time(Self::time_now()?, expiration_time)
                .map_err(|error| AtollWalletError::Input(error.to_string()))?;
        }

        Ok(self)
    }

    pub fn not_before(&mut self, reflection: &Reflection) -> AtollWalletResult<&mut Self> {
        if let Some(not_before) = reflection.reflect_string_or_undefined("notBefore") {
            let not_before = humantime::parse_rfc3339(&not_before)
                .or(Err(AtollWalletError::InvalidIS08601Timestamp(not_before)))?;
            self.0
                .set_expiration_time(Self::time_now()?, not_before)
                .map_err(|error| AtollWalletError::Input(error.to_string()))?;
        }

        Ok(self)
    }

    pub fn request_id(&mut self, reflection: &Reflection) -> AtollWalletResult<&mut Self> {
        if let Some(request_id) = reflection.reflect_string_or_undefined("requestId") {
            self.0.set_request_id(request_id.trim());
        }

        Ok(self)
    }

    pub fn resources(&mut self, reflection: &Reflection) -> AtollWalletResult<&mut Self> {
        let resource_raw = reflection
            .reflect_string_or_undefined("resources")
            .unwrap_or_default();

        if !resource_raw.is_empty() {
            let mut resources = Vec::<&str>::default();

            resource_raw
                .lines()
                .for_each(|line| resources.push(line.trim()));

            self.0.add_resources(&resources);
        }

        Ok(self)
    }
}
