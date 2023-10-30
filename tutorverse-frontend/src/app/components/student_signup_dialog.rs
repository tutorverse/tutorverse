use yew::prelude::*;

use crate::app::components::student_signup_form::StudentSignUpForm;

#[function_component(StudentSignUpDialog)]
pub fn student_signup_dialog() -> Html {
    html! {
        <>
            <button class="btn btn-outline-light btn-lg ms-4 me-4" type="button" data-bs-toggle="modal" data-bs-target="#student-signup-dialog">
                { "Student" }
            </button>
            <div class="modal fade" id="student-signup-dialog" aria-hidden="true" tabindex="-1">
                <div class="modal-dialog modal-dialog-centered">
                    <div class="modal-content">
                        <div class="p-5">
                            <StudentSignUpForm />
                        </div>
                    </div>
                </div>
            </div>
        </>
    }
}
