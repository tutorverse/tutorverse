use anchor_lang::prelude::*;

#[account]
#[derive(Default)]
pub struct Config {
    /// Version
    pub version: u8,

    /// The number of teacher profiles
    pub count_teachers: u32,

    /// The number of student profiles
    pub count_students: u32, //TODO unclear if needed
}

impl Config {
    pub const LEN: usize = std::mem::size_of::<Config>();
}