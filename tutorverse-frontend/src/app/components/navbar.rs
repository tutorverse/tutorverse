use std::rc::Rc;

use yew::prelude::*;
use yew_router::prelude::Link;

use crate::{
    app::{
        router::Route,
        services::wallet::{WalletAction, WalletContext},
    },
    util::DEBUG,
};

#[function_component(Navbar)]
pub fn app() -> Html {
    let wallet = use_context::<WalletContext>().expect("could not get wallet context");

    let all_pages_link = match DEBUG {
        true => html! {
            <li class="nav-item">
                <Link<Route> to={Route::TestAll} classes="nav-link">{ "All" }</Link<Route>>
            </li>
        },
        false => html! {},
    };

    let wallet_clone = wallet.clone();
    let on_click_connect = move |_| {
        let w = wallet_clone.clone();
        wasm_bindgen_futures::spawn_local(async move {
            let pubkey = w.connect().await.expect("could not connect to wallet");
            log::info!("Connected to wallet: {}", pubkey);

            w.dispatch(WalletAction::UpdatePubkey(Some(pubkey)))
        });
    };

    let wallet_clone = wallet.clone();
    let on_click_disconnect = move |_| {
        let w = wallet_clone.clone();
        wasm_bindgen_futures::spawn_local(async move {
            w.disconnect()
                .await
                .expect("could not disconnect from wallet");

            w.dispatch(WalletAction::UpdatePubkey(None));
        });
    };

    let mut connect_btn = html! {
        <button onclick={on_click_connect} class="btn btn-primary">{"Connect"}</button>
    };

    if wallet.is_connected() {
        connect_btn = html! {
            <button onclick={on_click_disconnect} class="btn btn-primary">{wallet.pubkey_short()}</button>
        }
    }

    html! {
        <nav class="navbar bg-body-tertiary navbar-expand-md">
            <div class="container-fluid">
                <div class="d-flex">
                    <Link<Route> to={Route::Home} classes="navbar-brand">{ "Tutorverse" } </Link<Route>>
                    <div class="input-group ms-2" style="width: 300px">
                        <input type="text" class="form-control" placeholder="Search" aria-label="Search" />
                        <span class="input-group-text" id="basic-addon2">
                            <i class="bi bi-search"></i>
                        </span>
                    </div>
                </div>
                <ul class="navbar-nav">
                    {all_pages_link}
                    <span class="p-2">
                        {"|"}
                    </span>
                    {connect_btn}
                </ul>
            </div>
        </nav>
    }
}
