pub mod passphrase_gen;
pub mod password_gen;

use thiserror::Error;


#[derive(Debug, Error, Eq, PartialEq)]
pub enum PassGenError {
    #[error("Length must be greater than 0")]
    InvalidLength,
    #[error("Token set must not be empty")]
    EmptyTokenSet,
}

pub trait PassGen<T> {
    fn new(length: usize, tokenset: Vec<T>, separator: Option<char>, word_case: Option<bool>) -> Result<Self, PassGenError>
    where
        Self: Sized;
    fn generate(&self) -> String;
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct PassphraseGen {
    length: usize,
    tokenset: Vec<String>,
    separator: char,
    word_case: bool,
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct PasswordGen {
    length: usize,
    tokenset: Vec<char>,
}