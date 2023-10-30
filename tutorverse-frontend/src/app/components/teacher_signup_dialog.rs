use yew::prelude::*;

use crate::app::components::teacher_signup_form::TeacherSignUpForm;

#[function_component(TeacherSignUpDialog)]
pub fn teacher_signup_dialog() -> Html {
    html! {
        <>
            <button class="btn btn-outline-light btn-lg ms-4 me-4" type="button" data-bs-toggle="modal" data-bs-target="#teacher-signup-dialog">
                { "Teacher" }
            </button>
            <div class="modal fade" id="teacher-signup-dialog" aria-hidden="true" tabindex="-1">
                <div class="modal-dialog modal-dialog-centered">
                    <div class="modal-content">
                        <div class="p-5">
                            <TeacherSignUpForm />
                        </div>
                    </div>
                </div>
            </div>
        </>
    }
}
