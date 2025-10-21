use crate::wallet::metamask::adapter::{ethereum_request, is_metamask_installed};
use serde::{Deserialize, Serialize};
use wasm_bindgen::JsValue;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WalletState {
    pub connected: bool,
    pub account: Option<String>,
    pub chain_id: Option<String>,
    pub error: Option<String>,
    pub loading: bool,
}

impl Default for WalletState {
    fn default() -> Self {
        Self {
            connected: false,
            account: None,
            chain_id: None,
            error: None,
            loading: false,
        }
    }
}

#[derive(Debug, Clone)]
pub struct MetaMaskWallet {
    pub state: WalletState,
}

impl Default for MetaMaskWallet {
    fn default() -> Self {
        Self {
            state: WalletState::default(),
        }
    }
}

impl MetaMaskWallet {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn is_installed(&self) -> bool {
        is_metamask_installed()
    }

    pub async fn connect(&mut self) -> Result<String, String> {
        if !self.is_installed() {
            return Err("MetaMask is not installed!".to_string());
        }

        self.state.loading = true;
        self.state.error = None;

        // Create request parameters
        let params = js_sys::Object::new();
        js_sys::Reflect::set(
            &params,
            &JsValue::from_str("method"),
            &JsValue::from_str("eth_requestAccounts"),
        )
        .map_err(|_| "Failed to set request method")?;

        // Request accounts
        match ethereum_request(params.into()).await {
            Ok(accounts) => {
                let accounts_array = js_sys::Array::from(&accounts);
                if accounts_array.length() > 0 {
                    let account = accounts_array
                        .get(0)
                        .as_string()
                        .unwrap_or_default()
                        .to_lowercase();

                    // Get chain ID
                    let chain_id = self.get_chain_id().await.ok();

                    self.state.connected = true;
                    self.state.account = Some(account.clone());
                    self.state.chain_id = chain_id;
                    self.state.loading = false;

                    Ok(account)
                } else {
                    self.state.loading = false;
                    Err("No accounts found".to_string())
                }
            }
            Err(e) => {
                let error_msg = js_sys::Reflect::get(&e, &JsValue::from_str("message"))
                    .ok()
                    .and_then(|v| v.as_string())
                    .unwrap_or_else(|| "Failed to connect wallet".to_string());

                self.state.error = Some(error_msg.clone());
                self.state.loading = false;
                Err(error_msg)
            }
        }
    }

    pub fn disconnect(&mut self) {
        self.state = WalletState::default();
    }

    pub async fn get_chain_id(&self) -> Result<String, String> {
        let params = js_sys::Object::new();
        js_sys::Reflect::set(
            &params,
            &JsValue::from_str("method"),
            &JsValue::from_str("eth_chainId"),
        )
        .map_err(|_| "Failed to set request method")?;

        match ethereum_request(params.into()).await {
            Ok(chain_id) => chain_id
                .as_string()
                .ok_or_else(|| "Invalid chain ID".to_string()),
            Err(_) => Err("Failed to get chain ID".to_string()),
        }
    }

    pub async fn switch_chain(&self, chain_id: &str) -> Result<(), String> {
        let params = js_sys::Object::new();
        js_sys::Reflect::set(
            &params,
            &JsValue::from_str("method"),
            &JsValue::from_str("wallet_switchEthereumChain"),
        )
        .map_err(|_| "Failed to set request method")?;

        let params_array = js_sys::Array::new();
        let chain_params = js_sys::Object::new();
        js_sys::Reflect::set(
            &chain_params,
            &JsValue::from_str("chainId"),
            &JsValue::from_str(chain_id),
        )
        .map_err(|_| "Failed to set chain ID")?;
        params_array.push(&chain_params);

        js_sys::Reflect::set(&params, &JsValue::from_str("params"), &params_array)
            .map_err(|_| "Failed to set params")?;

        ethereum_request(params.into())
            .await
            .map(|_| ())
            .map_err(|e| {
                js_sys::Reflect::get(&e, &JsValue::from_str("message"))
                    .ok()
                    .and_then(|v| v.as_string())
                    .unwrap_or_else(|| "Failed to switch chain".to_string())
            })
    }

    pub fn format_address(&self) -> String {
        if let Some(address) = &self.state.account {
            if address.len() > 10 {
                format!("{}...{}", &address[..6], &address[address.len() - 4..])
            } else {
                address.clone()
            }
        } else {
            String::new()
        }
    }

    pub fn get_chain_name(chain_id: &str) -> &'static str {
        match chain_id {
            "0x1" => "Ethereum Mainnet",
            "0x5" => "Goerli Testnet",
            "0xaa36a7" => "Sepolia Testnet",
            "0x89" => "Polygon",
            "0xa" => "Optimism",
            "0xa4b1" => "Arbitrum One",
            "0x38" => "BNB Chain",
            _ => "Unknown Network",
        }
    }

    pub async fn sign_message(&self, message: &str) -> Result<String, String> {
        if !self.state.connected || self.state.account.is_none() {
            return Err("Wallet not connected".to_string());
        }

        let params = js_sys::Object::new();
        js_sys::Reflect::set(
            &params,
            &JsValue::from_str("method"),
            &JsValue::from_str("personal_sign"),
        )
        .map_err(|_| "Failed to set request method")?;

        let params_array = js_sys::Array::new();
        params_array.push(&JsValue::from_str(message));
        params_array.push(&JsValue::from_str(self.state.account.as_ref().unwrap()));

        js_sys::Reflect::set(&params, &JsValue::from_str("params"), &params_array)
            .map_err(|_| "Failed to set params")?;

        match ethereum_request(params.into()).await {
            Ok(signature) => signature
                .as_string()
                .ok_or_else(|| "Invalid signature".to_string()),
            Err(e) => {
                let error_msg = js_sys::Reflect::get(&e, &JsValue::from_str("message"))
                    .ok()
                    .and_then(|v| v.as_string())
                    .unwrap_or_else(|| "Failed to sign message".to_string());
                Err(error_msg)
            }
        }
    }

    pub async fn sign_and_transmit_transaction(
        &self,
        to: &str,
        data: &str,
        value: &str,
        chain_id: u32,
    ) -> Result<String, String> {
        if !self.state.connected || self.state.account.is_none() {
            return Err("Wallet not connected".to_string());
        }

        // Convert chain_id to hex string
        let chain_id_hex = format!("0x{:x}", chain_id);

        // Check if we're on the correct chain
        let current_chain = self.get_chain_id().await?;
        if current_chain != chain_id_hex {
            // Switch to the correct chain
            self.switch_chain(&chain_id_hex).await?;
        }

        // Create transaction object
        let tx_params = js_sys::Object::new();
        js_sys::Reflect::set(
            &tx_params,
            &JsValue::from_str("from"),
            &JsValue::from_str(self.state.account.as_ref().unwrap()),
        )
        .map_err(|_| "Failed to set from address")?;

        js_sys::Reflect::set(&tx_params, &JsValue::from_str("to"), &JsValue::from_str(to))
            .map_err(|_| "Failed to set to address")?;

        js_sys::Reflect::set(
            &tx_params,
            &JsValue::from_str("data"),
            &JsValue::from_str(data),
        )
        .map_err(|_| "Failed to set data")?;

        js_sys::Reflect::set(
            &tx_params,
            &JsValue::from_str("value"),
            &JsValue::from_str(value),
        )
        .map_err(|_| "Failed to set value")?;

        // Create params array with transaction object
        let params_array = js_sys::Array::new();
        params_array.push(&tx_params);

        // Create request object
        let params = js_sys::Object::new();
        js_sys::Reflect::set(
            &params,
            &JsValue::from_str("method"),
            &JsValue::from_str("eth_sendTransaction"),
        )
        .map_err(|_| "Failed to set request method")?;

        js_sys::Reflect::set(&params, &JsValue::from_str("params"), &params_array)
            .map_err(|_| "Failed to set params")?;

        // Send transaction
        match ethereum_request(params.into()).await {
            Ok(tx_hash) => tx_hash
                .as_string()
                .ok_or_else(|| "Invalid transaction hash".to_string()),
            Err(e) => {
                let error_msg = js_sys::Reflect::get(&e, &JsValue::from_str("message"))
                    .ok()
                    .and_then(|v| v.as_string())
                    .unwrap_or_else(|| "Failed to send transaction".to_string());
                Err(error_msg)
            }
        }
    }
}
