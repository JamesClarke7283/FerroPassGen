pub mod passphrase_gen;
pub mod password_gen;

use thiserror::Error;

// # FerroPassGen
//
// FerroPassGen is a Rust library for generating strong and secure passwords and passphrases.
//
// ## Features
//
// - Generate passwords with customizable length and character sets.
// - Generate passphrases with customizable length, word list, separator, and word case.
// - Lightweight and fast password generation.
// - Reusable library for integrating password generation into other Rust projects.
//
// ## Usage
//
// To use FerroPassGen in your Rust project, add the following to your `Cargo.toml`:
//
// ```toml
// [dependencies]
// ferropassgen = "0.1.0"
// ```
//
// Then, import the necessary modules and structs in your Rust code:
//
// ```rust
// use ferropassgen::{PassGen, PasswordGen, PassphraseGen, PassGenError};
// ```
//
// For detailed usage examples, refer to the documentation of the specific modules and structs.

/// Represents an error that can occur during password generation.
///
/// # Variants
///
/// - `InvalidLength`: Indicates that the provided length is invalid (must be greater than 0).
/// - `EmptyTokenSet`: Indicates that the provided token set is empty.
#[derive(Debug, Error, Eq, PartialEq)]
pub enum PassGenError {
    /// Length must be greater than 0.
    #[error("Length must be greater than 0")]
    InvalidLength,
    /// Token set must not be empty.
    #[error("Token set must not be empty")]
    EmptyTokenSet,
}

/// Trait for password generators.
///
/// This trait defines the common functionality for password generators.
///
/// # Type Parameters
///
/// - `T`: The type of the token set elements.
pub trait PassGen<T> {
    /// Creates a new instance of the password generator.
    ///
    /// # Arguments
    ///
    /// - `length`: The desired length of the generated password.
    /// - `tokenset`: The set of tokens to choose from when generating the password.
    /// - `separator`: An optional separator character for joining tokens (only applicable to passphrases).
    /// - `word_case`: An optional boolean indicating whether to use uppercase words (only applicable to passphrases).
    ///
    /// # Errors
    ///
    /// Returns a `PassGenError` if the provided length is 0 or the token set is empty.
    ///
    /// # Examples
    ///
    /// ```
    /// use ferropassgen::{PassGen, PasswordGen};
    ///
    /// let length = 8;
    /// let tokenset = vec!['a', 'b', 'c', 'd', 'e'];
    /// let password_gen = PasswordGen::new(length, tokenset, None, None).unwrap();
    /// ```
    fn new(length: usize, tokenset: Vec<T>, separator: Option<char>, word_case: Option<bool>) -> Result<Self, PassGenError>
    where
        Self: Sized;

    /// Generates a password using the password generator.
    ///
    /// # Examples
    ///
    /// ```
    /// use ferropassgen::{PassGen, PasswordGen};
    ///
    /// let length = 8;
    /// let tokenset = vec!['a', 'b', 'c', 'd', 'e'];
    /// let password_gen = PasswordGen::new(length, tokenset, None, None).unwrap();
    /// let password = password_gen.generate();
    /// assert_eq!(password.len(), 8);
    /// ```
    fn generate(&self) -> String;
}

/// Struct representing a passphrase generator.
///
/// # Fields
///
/// - `length`: The desired length of the generated passphrase.
/// - `tokenset`: The set of words to choose from when generating the passphrase.
/// - `separator`: The character used to separate the words in the generated passphrase.
/// - `word_case`: A boolean indicating whether to use uppercase words in the generated passphrase.
#[allow(dead_code)]
#[derive(Debug)]
pub struct PassphraseGen {
    length: usize,
    tokenset: Vec<String>,
    separator: char,
    word_case: bool,
}

/// Struct representing a password generator.
///
/// # Fields
///
/// - `length`: The desired length of the generated password.
/// - `tokenset`: The set of characters to choose from when generating the password.
#[allow(dead_code)]
#[derive(Debug)]
pub struct PasswordGen {
    length: usize,
    tokenset: Vec<char>,
}