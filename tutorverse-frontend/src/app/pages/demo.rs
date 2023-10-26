use std::rc::Rc;

use yew::prelude::*;

use crate::app::services::wallet::WalletContext;

#[function_component(Demo)]
pub fn demo() -> Html {
    let wallet = use_context::<WalletContext>().expect("WalletContext not found");

    let not_connected = html! {
        <div>
            <h4>{"Hello, please connect to your wallet to continue!"}</h4>
        </div>
    };

    let connected = html! {
        <div>
            <h4>{format!("Welcome user: {}", wallet.pubkey_short())}</h4>
        </div>
    };

    let wallet_view = match wallet.is_connected() {
        true => connected,
        false => not_connected,
    };

    let has_no_role = html! {
        <div>
            <h4>{"You have not selected your role!"}</h4>
            <h5>{"Please select your role to continue!"}</h5>
            <div class="mt-4">
                <button classes="btn btn-outline-light btn-lg ms-4 me-4"> { "Teacher" } </button>
                <button classes="btn btn-outline-light btn-lg ms-4 me-4"> { "Student" } </button>
            </div>

        </div>
    };

    let has_role = html! {
        <div>
            <h4>{"You have selected your role!"}</h4>
        </div>
    };

    let role_view = match wallet.is_connected() {
        true => has_role,
        false => has_no_role,
    };

    html! {
        <div class="container">
            <div class="row justify-content-center mt-3">
                <div class="col-md-8">
                    <h1> {"Tutorverse Demo Page"} </h1>
                </div>
                <div class="col-md-10 mt-2">
                    {wallet_view}
                </div>
                <div class="col-md-10 mt-2">
                    {role_view}
                </div>
            </div>
        </div>
    }
}
