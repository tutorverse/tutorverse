use anyhow::{Context, Result};
use wasm_bindgen::JsCast;
use web_sys::{EventTarget, HtmlInputElement};
use yew::prelude::*;

use crate::{
    app::services::wallet::WalletContext, solana::contract_client::ContractClient,
    types::CreateStudentInstruction,
};

async fn create_student(wallet: WalletContext, student: CreateStudentInstruction) -> Result<()> {
    let client = ContractClient::default();

    let pk = wallet.pubkey().context("Wallet not connected")?;
    let mut tx = client.build_create_student_tx(student, &pk).await?;
    // phantom requires the blockhash to be set
    tx.message.recent_blockhash = client.inner.get_latest_blockhash().await?;

    wallet.sign_and_send_transaction(tx).await?;

    Ok(())
}

#[function_component(StudentSignUpForm)]
pub fn student_signup_form() -> Html {
    let wallet = use_context::<WalletContext>().expect("WalletContext not found");

    let student = use_state(|| CreateStudentInstruction::default());
    let student_clone = student.clone();

    let onclick = move |_| {
        let w = wallet.clone();
        let s = (*student_clone).clone();
        wasm_bindgen_futures::spawn_local(async move {
            match create_student(w, s.clone()).await {
                Ok(_) => {
                    log::info!("Student created");
                }
                Err(e) => {
                    log::error!("Error creating student: {:?}", e);
                }
            }
        })
    };

    let student_clone = student.clone();
    let onchange = move |e: Event| {
        let mut s: CreateStudentInstruction = (*student_clone).to_owned();

        // When events are created the target is undefined, it's only
        // when dispatched does the target get added.
        let target: Option<EventTarget> = e.target();
        // Events can bubble so this listener might catch events from child
        // elements which are not of type HtmlInputElement
        let input = target.and_then(|t| t.dyn_into::<HtmlInputElement>().ok());

        if let Some(input) = input {
            match input.name().as_str() {
                "title" => s.title = input.value(),
                "contact-info" => s.contact_info = input.value(),
                _ => {}
            }

            student_clone.set(s);
        }
    };

    html! {
        <div>
            <h1> {"Student Sign Up"} </h1>
            <input name="title" onchange={onchange.clone()} value={student.title.clone()}  type="text" class="form-control mt-4" placeholder="Title" aria-label="title" />
            <input name="contact-info" onchange={onchange.clone()} value={student.contact_info.clone()}  type="text" class="form-control mt-4" placeholder="Contact Info" aria-label="contact-info" />
            <button onclick={onclick} class="btn btn-outline-light btn-lg mt-4"> {"Sign Up"} </button>
        </div>
    }
}
