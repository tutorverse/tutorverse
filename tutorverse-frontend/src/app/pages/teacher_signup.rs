use yew::prelude::*;

use crate::app::components::teacher_signup_form::TeacherSignUpForm;

#[function_component(TeacherSignUpPage)]
pub fn teacher_signup_page() -> Html {
    html! {
        <div>
            <TeacherSignUpForm />
        </div>
    }
}
