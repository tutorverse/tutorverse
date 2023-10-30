use borsh::BorshSerialize;
use serde::{Deserialize, Serialize};
use yew::Properties;

#[derive(Debug, Serialize, Deserialize, BorshSerialize, Default, Clone)]
pub struct CreateStudentInstruction {
    pub title: String,
    pub contact_info: String,
}

#[derive(Debug, Serialize, Default, Clone, Properties, PartialEq)]
pub struct YewStudent {
    pub title: String,
    pub contact_info: String,
}
