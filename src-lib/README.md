# FerroPassGen

[![Crates.io](https://img.shields.io/crates/v/ferropassgen)](https://crates.io/crates/ferropassgen)
[![Downloads](https://img.shields.io/crates/d/ferropassgen.svg)](https://crates.io/crates/ferropassgen)
[![Documentation](https://docs.rs/ferropassgen/badge.svg)](https://docs.rs/ferropassgen)
[![License](https://img.shields.io/crates/l/ferropassgen)](https://crates.io/crates/ferropassgen)
[![Dependency Status](https://deps.rs/repo/github/JamesClarke7283/FerroPassGen/status.svg)](https://deps.rs/repo/github/JamesClarke7283/FerroPassGen)

`ferropassgen` is a Rust library for generating strong and secure passwords and
passphrases. It provides a flexible and customizable password generation
framework that can be easily integrated into other Rust projects.

## Features

- Generate strong passwords with customizable length and character sets
- Generate memorable passphrases with customizable length, word list, separator,
  and word case
- Lightweight and fast password generation
- Extensible and customizable password generation traits
- Well-documented API with examples and error handling

## Usage

To use `ferropassgen` in your Rust project, add the following to your
`Cargo.toml`:

```toml
[dependencies]
ferropassgen = "0.1.0"
```

Then, import the necessary modules and structs in your Rust code:

```rust
use ferropassgen::{PassGen, PasswordGen, PassphraseGen, PassGenError};
```

### Generating Passwords

To generate a password, create an instance of `PasswordGen` and call the
`generate` method:

```rust
let length = 16;
let charset = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789!@#$%^&*".chars().collect::<Vec<char>>();
let password_gen = PasswordGen::new(length, charset, None, None)?;
let password = password_gen.generate();
```

### Generating Passphrases

To generate a passphrase, create an instance of `PassphraseGen` and call the
`generate` method:

```rust
let length = 4;
let wordlist = vec!["apple", "banana", "cherry", "date", "elderberry"]
    .iter()
    .map(|&s| s.to_string())
    .collect::<Vec<String>>();
let separator = Some('_');
let word_case = Some(true);
let passphrase_gen = PassphraseGen::new(length, wordlist, separator, word_case)?;
let passphrase = passphrase_gen.generate();
```

### Error Handling

The `new` methods of `PasswordGen` and `PassphraseGen` return a
`Result<Self, PassGenError>`. Make sure to handle the potential errors
appropriately.

## Examples

Here are a few examples of using `ferropassgen` to generate passwords and
passphrases:

```rust
use ferropassgen::{PassGen, PasswordGen, PassphraseGen};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Generate a 20-character password with a custom character set
    let length = 20;
    let charset = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789".chars().collect::<Vec<char>>();
    let password_gen = PasswordGen::new(length, charset, None, None)?;
    let password = password_gen.generate();
    println!("Generated Password: {}", password);

    // Generate a passphrase with 5 words, separated by underscores, and in uppercase
    let length = 5;
    let wordlist = vec!["apple", "banana", "cherry", "date", "elderberry"]
        .iter()
        .map(|&s| s.to_string())
        .collect::<Vec<String>>();
    let separator = Some('_');
    let word_case = Some(true);
    let passphrase_gen = PassphraseGen::new(length, wordlist, separator, word_case)?;
    let passphrase = passphrase_gen.generate();
    println!("Generated Passphrase: {}", passphrase);

    Ok(())
}
```

## API Documentation

For detailed information about the `ferropassgen` API, including structs,
traits, and methods, please refer to the
[API documentation](https://docs.rs/ferropassgen).

## License

This project is licensed under the
[GNU Lesser General Public License v3.0](LICENSE).

## Acknowledgements

`ferropassgen` was inspired by the need for a flexible and customizable password
generation library in Rust. It builds upon the excellent work of the Rust
community and the libraries they have created.

Special thanks to the developers of the following libraries:

- [rand](https://crates.io/crates/rand) - Random number generation
- [thiserror](https://crates.io/crates/thiserror) - Error handling
