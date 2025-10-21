#[cfg(not(feature = "ssr"))]
use leptos::prelude::*;

#[cfg(not(feature = "ssr"))]
use super::wallet::MetaMaskWallet;

#[cfg(not(feature = "ssr"))]
#[derive(Clone)]
pub struct MetaMaskContext {
    pub wallet: StoredValue<MetaMaskWallet>,
}

#[cfg(not(feature = "ssr"))]
impl MetaMaskContext {
    pub fn new() -> Self {
        Self {
            wallet: StoredValue::new(MetaMaskWallet::new()),
        }
    }

    pub fn get_wallet(&self) -> MetaMaskWallet {
        self.wallet.get_value()
    }

    pub fn set_wallet(&self, wallet: MetaMaskWallet) {
        self.wallet.set_value(wallet);
    }
}

#[cfg(not(feature = "ssr"))]
pub fn provide_metamask_context() {
    provide_context(MetaMaskContext::new());
}

#[cfg(not(feature = "ssr"))]
pub fn use_metamask_context() -> Option<MetaMaskContext> {
    use_context::<MetaMaskContext>()
}
