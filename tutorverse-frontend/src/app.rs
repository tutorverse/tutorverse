use std::rc::Rc;

use yew::prelude::*;
use yew_router::{BrowserRouter, Switch};

use crate::app::{
    components::navbar::Navbar, pages::home::Home, router::Route, services::wallet::WalletProvider,
};

mod components;
mod pages;
mod router;
mod services;

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <WalletProvider>
            <BrowserRouter>
                <Navbar />
                <main>
                    <Switch<Route> render={router::switch} />
                </main>
            </BrowserRouter>
        </WalletProvider>
    }
}
