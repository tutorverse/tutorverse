use std::{cell::RefCell, rc::Rc, str::FromStr};

use anyhow::{anyhow, Context, Result};
use log::debug;
use solana_sdk::{pubkey::Pubkey, transaction::Transaction, wasm_bindgen};
use wasm_bindgen::JsValue;
use yew::prelude::*;
use yew::{Reducible, UseReducerHandle};

use crate::util::LocalStorage;

#[wasm_bindgen(module = "/js/phantom.js")]
extern "C" {
    fn tx_from_buffer(buf: &[u8]) -> JsValue;
}

fn reflect_get(target: &JsValue, key: &JsValue) -> Result<JsValue> {
    let result = js_sys::Reflect::get(target, key).map_err(|e| anyhow!("{:?}", e))?;
    debug!("reflect_get: {:?}", result);
    Ok(result)
}

pub type WalletContext = UseReducerHandle<Wallet>;

pub enum WalletAction {
    UpdatePubkey(Option<Pubkey>),
}

#[derive(Debug, Clone, PartialEq)]
pub struct Wallet {
    pubkey: Option<Pubkey>,
}

impl Reducible for Wallet {
    type Action = WalletAction;
    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        let next_wallet = match action {
            WalletAction::UpdatePubkey(pubkey) => Self { pubkey },
        };
        Rc::new(next_wallet)
    }
}

impl Wallet {
    pub fn new(pubkey: Option<Pubkey>) -> Self {
        Self { pubkey }
    }

    pub fn init() -> Self {
        let pubkey = Self::get_pubkey().ok();
        Self::new(pubkey)
    }

    pub fn is_connected(&self) -> bool {
        self.pubkey.is_some()
    }

    pub fn pubkey(&self) -> Option<Pubkey> {
        self.pubkey.clone()
    }

    pub fn is_teacher(&self) -> bool {
        false
    }

    pub fn is_student(&self) -> bool {
        false
    }

    pub fn has_role(&self) -> bool {
        self.is_teacher() || self.is_student()
    }

    pub fn pubkey_str(&self) -> String {
        let pk = self.pubkey();
        match pk {
            Some(p) => p.to_string(),
            None => "".to_string(),
        }
    }

    pub fn pubkey_short(&self) -> String {
        let pk_str = self.pubkey_str();
        if pk_str.len() < 8 {
            return pk_str;
        }

        let chars: Vec<char> = pk_str.chars().collect();
        let l = chars.len();
        let start: String = chars[..4].iter().collect();
        let end: String = chars[l - 4..l].iter().collect();
        let pk_short = format!("{start}...{end}");
        pk_short
    }

    pub async fn disconnect(&self) -> Result<()> {
        debug!("disconnect");

        Self::delete_pubkey()?;

        //*self.pubkey.borrow_mut() = None;

        Ok(())
    }

    pub async fn connect(&self) -> Result<Pubkey> {
        debug!("connect_to_wallet");
        let window = web_sys::window().context("could not get window")?;
        if let Some(solana) = window.get("solana") {
            let is_phantom = reflect_get(&*solana, &wasm_bindgen::JsValue::from_str("isPhantom"))?;

            if is_phantom == JsValue::from(true) {
                let connect_str = wasm_bindgen::JsValue::from_str("connect");
                let connect: js_sys::Function = reflect_get(&*solana, &connect_str)?.into();

                log::debug!("{:?}", connect.to_string());

                let resp = connect.call0(&solana).map_err(|err| anyhow!("{err:?}"))?;
                let promise = js_sys::Promise::resolve(&resp);

                let result = wasm_bindgen_futures::JsFuture::from(promise)
                    .await
                    .map_err(|err| anyhow!("{err:?}"))?;

                log::debug!("{:?}", result);

                let pubkey_str = wasm_bindgen::JsValue::from_str("publicKey");
                let pubkey_obj: js_sys::Object = reflect_get(&result, &pubkey_str)?.into();

                let bn_str = wasm_bindgen::JsValue::from_str("toString");
                let to_string_fn: js_sys::Function = reflect_get(&pubkey_obj, &bn_str)?.into();

                log::debug!("pubkey_obj: {:?}", to_string_fn.call0(&pubkey_obj));

                let pubkey = to_string_fn
                    .call0(&pubkey_obj)
                    .map_err(|err| anyhow!("{:?}", err))?;

                let public_key = Pubkey::from_str(
                    &pubkey
                        .as_string()
                        .context("could not convert pubkey to string")?,
                )?;

                Self::save_pubkey(public_key)?;

                log::debug!("pubkey: {:?}", public_key);

                return Ok(public_key);
            }

            debug!("isPhantom: {:?}", is_phantom);
        }

        Err(anyhow!("could not connect to wallet"))
    }

    fn save_pubkey(pubkey: Pubkey) -> Result<()> {
        let storage = LocalStorage::new()?;
        storage.set("pubkey", &pubkey.to_string())?;
        Ok(())
    }

    fn delete_pubkey() -> Result<()> {
        let storage = LocalStorage::new()?;
        storage.remove("pubkey")?;
        Ok(())
    }

    fn get_pubkey() -> Result<Pubkey> {
        let storage = LocalStorage::new()?;
        let pubkey_str = storage.get("pubkey")?;
        let pubkey = Pubkey::from_str(&pubkey_str)?;
        Ok(pubkey)
    }

    pub async fn sign_and_send_transaction(&self, tx: Transaction) -> Result<()> {
        let solana = web_sys::window()
            .context("could not get window")?
            .get("solana")
            .context("could not get solana")?;
        let sign_tx_str = wasm_bindgen::JsValue::from_str("signAndSendTransaction");
        let sign_tx_method: js_sys::Function = reflect_get(&*solana, &sign_tx_str)?.into();

        let tx_bytes = bincode::serialize(&tx)?;

        let tx_js = tx_from_buffer(&tx_bytes);

        let resp = sign_tx_method
            .call1(&solana, &tx_js)
            .map_err(|err| anyhow!("error signing and sending transaction: {:?}", err))?;

        let promise = js_sys::Promise::resolve(&resp);
        let result = wasm_bindgen_futures::JsFuture::from(promise)
            .await
            .map_err(|err| anyhow!("error signing and sending transaction: {:?}", err))?;

        log::debug!("result: {:?}", result);

        Ok(())
    }
}

#[derive(Properties, Debug, PartialEq)]
pub struct WalletProviderProps {
    #[prop_or_default]
    pub children: Html,
}

#[function_component]
pub fn WalletProvider(props: &WalletProviderProps) -> Html {
    let wallet = use_reducer(|| Wallet::init());

    html! {
        <ContextProvider<WalletContext> context={wallet}>
            {props.children.clone()}
        </ContextProvider<WalletContext>>
    }
}
