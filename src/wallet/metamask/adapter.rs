use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "ethereum"], js_name = request, catch)]
    pub async fn ethereum_request(params: JsValue) -> Result<JsValue, JsValue>;

    #[wasm_bindgen(js_namespace = ["window", "ethereum"], js_name = isMetaMask, thread_local)]
    static IS_META_MASK: bool;

    #[wasm_bindgen(js_namespace = ["window", "ethereum"], js_name = on)]
    pub fn ethereum_on(event: &str, callback: &Closure<dyn FnMut(JsValue)>);

    #[wasm_bindgen(js_namespace = ["window", "ethereum"], js_name = removeListener)]
    pub fn ethereum_remove_listener(event: &str, callback: &Closure<dyn FnMut(JsValue)>);

    #[wasm_bindgen(js_namespace = ["window", "ethereum"], js_name = selectedAddress, getter)]
    pub fn selected_address() -> Option<String>;

    #[wasm_bindgen(js_namespace = ["window", "ethereum"], js_name = chainId, getter)]
    pub fn chain_id() -> Option<String>;
}

pub fn is_metamask_installed() -> bool {
    if let Some(window) = web_sys::window() {
        let ethereum = js_sys::Reflect::get(&window, &JsValue::from_str("ethereum")).ok();
        if let Some(ethereum) = ethereum {
            !ethereum.is_undefined() && !ethereum.is_null()
        } else {
            false
        }
    } else {
        false
    }
}
